# Antithesis Rust Instrumentation Shim

This library provides methods for Rust programs to compile with coverage instrumentation for the Antithesis platform.

For general usage guidance, see the [Antithesis Rust Instrumentation Documentation](https://antithesis.com/docs/using_antithesis/sdk/rust/instrumentation/)

## Platform support

This crate builds for Linux targets only. The Antithesis platform runs Linux only, and this crate links against libvoidstar, a Linux shared library.

On all other targets, the build stops with a message that tells you how to correct your integration. The crate does not build to nothing, because a build that does nothing hides a bad integration. You can then send a binary to Antithesis with no instrumentation in it, and get no coverage data.

To keep your build possible on macOS and on Windows, put this crate behind a feature that you control. The next section gives the steps.

## How to integrate this crate

Do these three steps.

### 1. Add a feature that you control

Add an `antithesis` feature to your `Cargo.toml`, and make the dependency optional. Declare the dependency for Linux targets only, so that Cargo does not try to build it on other targets:

```toml
[features]
antithesis = ["dep:antithesis-instrumentation"]

[target.'cfg(target_os = "linux")'.dependencies]
antithesis-instrumentation = { version = "0.1", optional = true }
```

The name `antithesis` is an example. Use any name that you prefer.

The two parts do different work:

- The `[target.'cfg(target_os = "linux")']` table keeps this crate out of the dependency graph for macOS, for Windows, and for all other targets. This part stops the build failure.
- `optional = true` keeps this crate out of your usual Linux builds. This part is not necessary, but it is better. The `#[cfg]` gate in step 2 acts on your Rust code only. Cargo does not read your Rust code, so a dependency that is not optional is built for each `cargo build` and `cargo test` on Linux. The build script runs, and it compiles the C shim, even when the gate removes the `use` and nothing links the shim.

### 2. Gate the use of the crate on the target and on your feature

This crate has no public API. Your code must refer to it one time, so that the linker keeps the shim. Add these two lines to your `src/main.rs` or your `src/lib.rs`:

```rust
#[cfg(all(target_os = "linux", feature = "antithesis"))]
use antithesis_instrumentation as _;
```

Both conditions are necessary:

- `target_os = "linux"` keeps the crate out of the build for other targets.
- `feature = "antithesis"` keeps the crate out of your usual Linux builds. Instrumentation makes your program slower, so use it only for Antithesis.

### 3. Enable the feature only when you build for Antithesis

In your Antithesis `Dockerfile`, or in the build script that makes the image, add `--features antithesis` to the instrumented build:

```bash
cargo build --features antithesis \
  --config 'build.target = "x86_64-unknown-linux-gnu"' \
  --config 'target.x86_64-unknown-linux-gnu.rustflags = [
    "-Ccodegen-units=1",
    "-Cpasses=sancov-module",
    "-Cllvm-args=-sanitizer-coverage-level=3",
    "-Cllvm-args=-sanitizer-coverage-trace-pc-guard",
    "-Clink-args=-Wl,--build-id"
  ]'
```

For the current flags, and for the symbols that you must send to Antithesis, see the [Antithesis Rust Instrumentation Documentation](https://antithesis.com/docs/using_antithesis/sdk/rust/instrumentation/).

Leave the feature off everywhere else. Your usual `cargo build`, `cargo test`, and `cargo clippy` commands then work on all platforms, because Cargo does not put this crate in the dependency graph.

### If the build stops

A message that starts with `antithesis-instrumentation builds for Linux targets only` shows that the crate is in the dependency graph for a target that is not Linux. Look for one of these causes:

- The dependency is in the usual `[dependencies]` table. Move it to `[target.'cfg(target_os = "linux")'.dependencies]`. This is the most usual cause.
- A dependency of your crate has this crate in its own `[dependencies]` table. Then you must correct that crate, or make your dependency on it optional.
- Your feature is on by default, or another feature turns it on. Remove `antithesis` from `default`, and from all other feature lists. Note that the feature alone does not cause this failure if the target table is correct.
- A different crate in your workspace turns on the feature. Cargo combines the features of all members in a workspace build. Check each member.

## How it works

When you compile your code with the `-sanitizer-coverage-trace-pc-guard` from the documentation above, the Rust compiler adds two callbacks (`__sanitizer_cov_trace_pc_guard_init` and `__sanitizer_cov_trace_pc_guard`) at various points in your code. The former on load and the latter once per basic block.

This crate creates shim implementations for those two functions that detect whether you're running in Antithesis, and if so, forward the calls to the Antithesis system. When running outside of Antithesis, the shims become no-ops.

The shims come from `antithesis_instrumentation.h`, which is vendored from the [Antithesis C++ SDK](https://github.com/antithesishq/antithesis-sdk-cpp) into `vendor/` and compiled by `build.rs`. Vendoring it keeps builds offline- and hermetic-friendly: nothing is downloaded at build time.

## Updating the vendored header

Set `CPP_SDK_VERSION` at the top of [`scripts/update-vendored-header.sh`](scripts/update-vendored-header.sh) to the desired `antithesis-sdk-cpp` tag and run it:

```sh
./scripts/update-vendored-header.sh
```

Then run `cargo build` to confirm the new header still compiles, and add a `CHANGELOG.md` entry.
