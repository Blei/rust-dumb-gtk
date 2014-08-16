# FIXME: allow-bitfields results in incorrect code, but not specifying it results in
# a failed compilation...
# FIXME: try to break this huge ffi module up into multiple ones, one for glib, one for
# cairo, ...
BINDGEN_OPTS = `pkg-config --cflags-only-I --libs-only-l gtk+-3.0 gstreamer-1.0` \
	       -I/usr/lib/clang/3.4.2/include \
	       -allow-bitfields -builtins

.PHONY: lib clean

lib: src/ffi.rs
	cargo build

src/ffi.rs: generated_from.h
	rust-bindgen -o $@ ${BINDGEN_OPTS} $<

clean:
	rm -rf src/ffi.rs target
