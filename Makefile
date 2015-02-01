# FIXME: try to break this huge ffi module up into multiple ones, one for glib, one for
# cairo, ...
BINDGEN_OPTS = `pkg-config --cflags-only-I --libs-only-l gtk+-3.0 gstreamer-1.0` \
	       -I/usr/lib/clang/3.5.1/include -builtins

.PHONY: lib clean

lib: src/ffi.rs
	cargo build

src/ffi.rs: generated_from.h
	rust-bindgen -o $@ ${BINDGEN_OPTS} $<

clean:
	rm -rf src/ffi.rs target
