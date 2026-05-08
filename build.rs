use std::env;
use std::process::Command;

const CPP_SDK_VERSION: &str = "0.4.8";

fn main() {
    // Get antithesis_instrumentation.h from the antithesis-sdk-cpp github
    let out_dir = env::var("OUT_DIR").unwrap();
    let header = format!("{out_dir}/antithesis_instrumentation.h");

    let url = format!(
        "https://raw.githubusercontent.com/antithesishq/antithesis-sdk-cpp/\
         {CPP_SDK_VERSION}/antithesis_instrumentation.h"
    );
    let curl = Command::new("curl")
        .args(["-fsSL", "-o", &header, &url])
        .status()
        .expect("failed to spawn curl");
    assert!(curl.success(), "curl exited {curl} (url: {url})");

    // Compile libantithesis_instrumentation.a from antitithesis_instrumentation.c
    cc::Build::new()
        .file("src/antithesis_instrumentation.c")
        .include(&out_dir)
        .opt_level(3)
        .compile("antithesis_instrumentation");

    // Rerun hints
    println!("cargo::rerun-if-changed=src/antithesis_instrumentation.c");
    println!("cargo::rerun-if-changed=build.rs");
}
