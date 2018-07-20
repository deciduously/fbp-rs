# fbp-rs
Rust program for reading and writing Factorio blueprints.

This really doesn't do much right now, it was more an exercise than anything.  I'm not planning to actively work on this unless I get really bored.  If you want to implement your own Factorio/Rust thing, by all means take the structs I defined for serde, that's the most tedious part.
## Dependencies
* Stable rust 1.26.0+
## Usage
This app is organized into a library`fbp-rs` and a binary `fbp-tool`.  To use it invoke `cargo run --bin fbp-tool` or simply `cargo run`.  It will use [resource/balancer.txt](https://github.com/deciduously/fbp-rs/blob/master/resource/balancer.txt) if invoked without arguments, or attempt to use the first argument given.  I've included [resource/balancer.json](https://github.com/deciduously/fbp-rs/blob/master/resource/balancer.json) as a reference for what json this example blueprint decodes to before serializing into a Rust struct.

Right now, it'll spit back the string and do its very best to show you a preview in the console.  It's only ok at that.

Only blueprints containing a single blueprint object are supported right now - haven't gotten to blueprint books yet.

Actually, it doens't really work on much of anything beyond the arbitrary sample I chose.  types::Grid.grid_coords() is disgraceful and bugged to shite.

Stay tuned for hopefully something useful here, eventually.
