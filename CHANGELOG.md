# Changelog

## Unreleased

A build for a target that is not Linux now stops with a clear message that tells you how to integrate this crate. Before, it stopped with a `curl`, `cc`, or `invalid Mach-O section specifier` error that hid the cause. The README documents the integration: an optional dependency that you declare for Linux targets only, behind a feature that you control.

## 0.1.0 - 2026-05-07

Initial version