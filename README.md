# Rust-Lean Integration

This project aims to provide a robust set of tools for
interoperability between the Rust and Lean programming
languages.

The main features under development are:

 * Lean Cargo support

  We provide support for building Lean packages using a fork of
  [Cargo](https://doc.rust-lang.org/cargo/), the Rust Package Manager.

 * Rust package for Lean Runtime

  We provide a Rust package that allows low-level functions for creating
  Lean objects with reference counting.

Once these efforts are sufficiently mature, the plan is to explore generating
wrapper code to make it easier to invoke Lean from Rust and vice versa.

At this point, the tools are not mature and very little actually works, and
so use of Rust-Lean integration is not recommended.

## Cargo support

Cargo support is done through a fork that adds a new feature, `extern-build`,
which provides a mechanism for integrating external build tools into Cargo.  With
this extension, we need to provide a command line executable, `cargobuild-lean`
that serves as a bridge between the Lean compiler and cargo itself.
