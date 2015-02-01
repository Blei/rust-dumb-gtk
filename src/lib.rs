#![crate_name = "gtk"]
#![allow(non_camel_case_types)]

extern crate libc;

use std::ffi as r_ffi;
use std::mem;
use std::ptr;
use std::str;

use ffi::*;

pub mod ffi;

unsafe fn init_something_with_args(args: Vec<String>,
        f: unsafe extern "C" fn(*mut libc::c_int, *mut *mut *mut i8)) -> Vec<String> {
    // This vec is used to store the CStrings while we make them available to the ffi
    // for parsing.
    let c_args: Vec<r_ffi::CString> =
        args.iter().map(|s| { r_ffi::CString::from_slice(s.as_bytes()) }).collect();
    let mut args_n: i32 = args.len() as i32;
    let mut args_v =
        libc::malloc((std::mem::size_of::<*mut i8>() * args.len() + 1) as libc::size_t)
        as *mut *mut libc::c_char;

    for i in 0..args_n {
        // This is so unsafe... but we know that the c_args array
        // lives longer than the containing args_vec backing memory
        *args_v.offset(i as isize) = c_args[i as usize].as_slice_with_nul().as_ptr() as *mut _;
    }

    f(&mut args_n, &mut args_v);

    let mut new_args = Vec::new();
    {
        for i in 0..args_n {
            let arg = *args_v.offset(i as isize) as *const libc::c_char;
            let slice = r_ffi::c_str_to_bytes(&arg);
            new_args.push(str::from_utf8(slice).unwrap().to_string());
        }
    }

    libc::free(mem::transmute(args_v));

    new_args.into_iter().collect()
}

/// Returns the modified command line arguments
pub unsafe fn gtk_init_with_args_2(args: Vec<String>) -> Vec<String> {
    init_something_with_args(args, gtk_init)
}

pub unsafe fn gst_init_with_args(args: Vec<String>) -> Vec<String> {
    // clang deficiency, doesn't correctly encode `**argv[]` params
    init_something_with_args(args, gst_init)
}

pub unsafe fn g_signal_connect(instance: gpointer, detailed_signal: *const gchar,
                               c_handler: GCallback, data: gpointer) -> gulong {
    g_signal_connect_data(instance, detailed_signal, c_handler,
                          data, None, 0)
}

pub struct GListIterator<'a> {
    current: Option<&'a Struct__GList>,
}

impl <'a> GListIterator<'a> {
    pub fn new(glist: &'a GList) -> GListIterator<'a> {
        GListIterator {
            current: Some(glist)
        }
    }
}

impl <'a> Iterator for GListIterator<'a> {
    type Item = gpointer;

    fn next(&mut self) -> Option<gpointer> {
        if self.current.is_none() {
            return None;
        }

        let next = self.current.unwrap().next;
        if next != ptr::null_mut() {
            unsafe {
                self.current = mem::transmute(next);
                Some((*next).data)
            }
        } else {
            self.current = None;
            None
        }
    }
}
