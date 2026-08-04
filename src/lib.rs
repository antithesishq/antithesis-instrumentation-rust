// This file contains glue code that includes the (C++-based .a file)
// And makes sure the linker doesn't remove it eagerly.
//
// This crate builds for Linux targets only. On all other targets it stops the
// build with the message below, instead of doing nothing quietly. The
// `target_os` gates keep the ELF-only `.init_array` section from adding a
// second, less clear error to that message.

#[cfg(not(target_os = "linux"))]
compile_error!(
    r#"antithesis-instrumentation builds for Linux targets only.

The Antithesis platform runs Linux only, and this crate links against
libvoidstar, a Linux shared library. There is no useful build for other targets.

Do not depend on this crate unconditionally. Put it behind a feature that you
control, and declare the dependency for Linux targets only:

    # Cargo.toml
    [features]
    antithesis = ["dep:antithesis-instrumentation"]

    [target.'cfg(target_os = "linux")'.dependencies]
    antithesis-instrumentation = { version = "0.1", optional = true }

Then refer to the crate at one place in your own code:

    // src/main.rs or src/lib.rs
    #[cfg(all(target_os = "linux", feature = "antithesis"))]
    use antithesis_instrumentation as _;

Enable the feature only when you build for Antithesis:

    cargo build --target x86_64-unknown-linux-gnu --features antithesis

For the complete steps, see:
https://github.com/antithesishq/antithesis-instrumentation-rust#how-to-integrate-this-crate"#
);

#[cfg(target_os = "linux")]
#[link(name = "antithesis_instrumentation", kind = "static")]
unsafe extern "C" {
    fn antithesis_load_libvoidstar();
}

#[cfg(target_os = "linux")]
#[used]
#[unsafe(link_section = ".init_array")]
static _ANTITHESIS_INIT: unsafe extern "C" fn() = antithesis_load_libvoidstar;
