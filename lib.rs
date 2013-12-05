#[feature(globs)];
#[ link(name = "gtk",
        package_id = "gtk") ];

extern mod extra;

use std::cast;
use std::ptr;

use extra::c_vec;

use ffi::*;

pub mod ffi;

unsafe fn init_something_with_args(args: ~[~str],
        f: extern "C" unsafe fn(*mut std::libc::c_int, *mut *mut *mut i8)) -> ~[~str] {
    let mut c_args = args.map(|s| { s.to_c_str() });
    let mut args_n: i32 = args.len() as i32;
    let mut args_v = cast::transmute(
        std::libc::malloc((std::mem::size_of::<*i8>() * args.len() + 1) as std::libc::size_t));

    {
        let mut args_vec = c_vec::CVec::new(args_v, args.len());
        for i in std::iter::range(0, args_n as uint) {
            // This is so unsafe... but we know that the c_args array
            // lives longer than the containing args_vec backing memory
            c_args[i].with_mut_ref(|ptr| {
                *args_vec.get_mut(i) = ptr;
            });
        }
    }

    f(&mut args_n, &mut args_v);

    let mut new_args = ~[];
    {
        let args_vec2 = c_vec::CVec::new(args_v, args_n as uint);
        for i in std::iter::range(0, args_n as uint) {
            new_args.push(std::str::raw::from_c_str(&**args_vec2.get(i)));
        }
    }

    std::libc::free(cast::transmute(args_v));

    new_args
}

/// Returns the modified command line arguments
pub unsafe fn gtk_init_with_args(args: ~[~str]) -> ~[~str] {
    init_something_with_args(args, gtk_init)
}

pub unsafe fn gst_init_with_args(args: ~[~str]) -> ~[~str] {
    // clang deficiency, doesn't correctly encode `**argv[]` params
    let my_gst_init: extern "C" fn(*mut std::libc::c_int, *mut *mut *mut std::libc::c_schar) =
        cast::transmute(gst_init);
    init_something_with_args(args, my_gst_init)
}

pub unsafe fn g_signal_connect(instance: gpointer, detailed_signal: *gchar, c_handler: GCallback,
                    data: gpointer) -> gulong {
    g_signal_connect_data(instance, detailed_signal, c_handler,
        data, cast::transmute(0), 0)
}

pub struct GListIterator<'a> {
    priv current: Option<&'a Struct__GList>,
}

impl <'a> GListIterator<'a> {
    pub fn new(glist: &'a GList) -> GListIterator<'a> {
        GListIterator {
            current: Some(glist)
        }
    }
}

impl <'a> Iterator<gpointer> for GListIterator<'a> {
    fn next(&mut self) -> Option<gpointer> {
        if self.current.is_none() {
            return None;
        }

        let next = self.current.unwrap().next;
        if next != ptr::mut_null() {
            unsafe {
                self.current = cast::transmute(next);
                Some((*next).data)
            }
        } else {
            self.current = None;
            None
        }
    }
}

fn g_value_new(typ: GType) -> GValue {
    unsafe {
        let mut g_value: GValue = std::unstable::intrinsics::uninit();
        g_value_init(&mut g_value, typ);
        g_value
    }
}
