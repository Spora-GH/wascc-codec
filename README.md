[![crates.io](https://img.shields.io/crates/v/wascc-codec.svg)](https://crates.io/crates/wascc-codec)
![Rust](https://github.com/wascc/wascc-codec/workflows/Rust/badge.svg)
![license](https://img.shields.io/crates/l/wascc-codec.svg)
[![documentation](https://docs.rs/wascc-codec/badge.svg)](https://docs.rs/wascc-codec)

# waSCC Codec

The _WebAssembly Secure Capabilities Connector_ (waSCC) **codec** contains a few basic primitives that are
required to support the interaction between a waSCC host runtime and capability providers that conform
to the Rust trait and the C **FFI**. This crate is _not_ to be used for shared data types or wire
protocol - that problem should be dealt with using [widl](https://github.com/wapc) schemas and code generation.

