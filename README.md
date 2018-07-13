# fbp-rs

## Dependencies
* Stable rust 1.26.0+
## Usage
This app is organized into a library`fbp-rs` and a binary `fbp-tool`.  To use it invoke `cargo run --bin fbp-tool` or simply `cargo run`.  It will use [resource/balancer.txt](https://github.com/deciduously/fbp-rs/blob/master/resource/balancer.txt) if invoked without arguments, or attempt to use the first argument given.  I've included [resource/balancer.json](https://github.com/deciduously/fbp-rs/blob/master/resource/balancer.json) as a reference for what json this example blueprint decodes to before serializing into a Rust struct.

Stay tuned for hopefully something useful here, eventually.
