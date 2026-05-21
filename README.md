# Antithesis Rust Instrumentation Shim

This library provides methods for Rust programs to compile with coverage instrumentation for the Antithesis platform.

For general usage guidance, see the [Antithesis Rust Instrumentation Documentation](https://antithesis.com/docs/using_antithesis/sdk/rust/instrumentation/)

## How it works

When you compile your code with the `-sanitizer-coverage-trace-pc-guard` from the documentation above, the Rust compiler adds two callbacks (`__sanitizer_cov_trace_pc_guard_init` and `__sanitizer_cov_trace_pc_guard`) at various points in your code. The former on load and the latter once per basic block.

This crate creates shim implementations for those two functions that detect whether you're running in Antithesis, and if so, forward the calls to the Antithesis system. When running outside of Antithesis, the shims become no-ops.