// This file contains glue code that includes the (C++-based .a file)
// And makes sure the linker doesn't remove it eagerly.

#[link(name = "antithesis_instrumentation", kind = "static")]
unsafe extern "C" {
    fn antithesis_load_libvoidstar();
}

#[used]
#[unsafe(link_section = ".init_array")]
static _ANTITHESIS_INIT: unsafe extern "C" fn() = antithesis_load_libvoidstar;
