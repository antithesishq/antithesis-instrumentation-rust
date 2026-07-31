use std::env;

// antithesis_instrumentation.h is vendored in vendor/ rather than downloaded at build
// time; refresh it with scripts/update-vendored-header.sh.
const VENDOR_DIR: &str = "vendor";

fn main() {
    // Rerun hints
    println!("cargo::rerun-if-changed={VENDOR_DIR}/antithesis_instrumentation.h");
    println!("cargo::rerun-if-changed=src/antithesis_instrumentation.c");
    println!("cargo::rerun-if-changed=build.rs");

    // This crate builds for Linux targets only. Stop here on every other
    // target: `cc` below would fail with an error that hides the real problem.
    // `src/lib.rs` then stops the build with a clear message that tells the user
    // how to integrate this crate.
    if env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("linux") {
        return;
    }

    // Compile libantithesis_instrumentation.a from antitithesis_instrumentation.c
    cc::Build::new()
        .file("src/antithesis_instrumentation.c")
        .include(VENDOR_DIR)
        .opt_level(3)
        .compile("antithesis_instrumentation");
}
