extern crate base64;
extern crate flate2;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod types;

use base64::decode;
use flate2::read::GzDecoder;
use std::io::{self, prelude::*};
use types::*;

// returns a Json string from the compressed Blueprint
fn decode_blueprint(bp: &str) -> io::Result<String> {
    // skip the version byte - it's always "0" in factorio 0.15 and 0.16
    let encoded = &bp[1..];

    // base64 decode
    let decoded = decode(encoded).expect("Could not base64 decode blueprint");

    println!("{:?}", &decoded);
    // decompress with zlib deflate
    // Note - you're getting "invalid gzip header" when even just pasing ing "random str sdata".as_bytes()
    let mut bp_decoder = GzDecoder::new(&decoded[..]);
    let mut json_string = String::new();
    bp_decoder.read_to_string(&mut json_string)?;
    println!("{:?}", json_string);
    Ok(json_string)
}

fn serialize_blueprint(json: &str) -> io::Result<Container> {
    let ret: Container = serde_json::from_str(json).expect("Could not deserialize json");
    Ok(ret)
}

#[cfg(test)]
mod tests {
    use std::{
        fs::File, io::{BufReader, Read}, path::Path,
    };
    #[test]
    fn test_decode_blueprint() {
        use super::decode_blueprint;

        // Grab the sample blueprint
        let bp_file =
            File::open(Path::new("./resource/balancer.txt")).expect("Could not open balancer.txt");
        let mut bp_reader = BufReader::new(bp_file);
        let mut bp_string = String::new();
        bp_reader
            .read_to_string(&mut bp_string)
            .expect("Could not read balancer.txt");

        // Grab the expected blueprint JSON
        let decoded_target_file = File::open(Path::new("./resource/balancer.json"))
            .expect("Could not open balancer.json");
        let mut json_reader = BufReader::new(decoded_target_file);
        let mut json_string = String::new();
        json_reader
            .read_to_string(&mut json_string)
            .expect("Could not read balancer.json");

        // you're getting "invalid gzip header"
        let attempt = decode_blueprint(&bp_string).unwrap();
        assert_eq!(attempt, json_string);
    }

    #[test]
    fn test_serialize_blueprint() {
        use super::serialize_blueprint;

        let decoded_target_file = File::open(Path::new("./resource/balancer.json"))
            .expect("Could not open balancer.json");
        let mut json_reader = BufReader::new(decoded_target_file);
        let mut json_string = String::new();
        json_reader
            .read_to_string(&mut json_string)
            .expect("Could not read balancer.json");
        let serialized = serialize_blueprint(&json_string).unwrap();

        assert!(serialized.ok());
    }
}
