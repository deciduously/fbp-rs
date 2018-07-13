use base64::decode;
use flate2::read::ZlibDecoder;
use serde_json;
use std::io::{self, prelude::*};
use types::*;

// Grid type to coerce the entity list into
// Factorio blueprints encompass a 14x14 grid, with 0,0 being the center
// See if we can't get this into an array - need to get Entity to by Copy
// the Strings are holding you up
#[derive(Debug)]
pub struct Grid {
    pub cells: Vec<Vec<Option<Entity>>>,
}

impl Grid {
    // TODO real error
    pub fn from(c: Container) -> Result<Self, String> {
        let entities = c.blueprint.entities;
        let mut cells = vec![vec![None; 14]; 14];
        for e in &entities {
            // replace the proper cell with the entity
            let pos = &e.position;
            // this is NOT correct - your raw positions need to be translated to grid positions
            cells[pos.x as usize][pos.y as usize] = Some(e.clone());
        }
        Ok(Grid { cells: cells })
    }
}

// returns a Json string from the compressed Blueprint
fn decode_blueprint(bp: &str) -> io::Result<String> {
    // skip the version byte - it's always "0" in factorio 0.15 and 0.16
    let encoded = &bp[1..];

    // base64 decode
    let decoded = decode(encoded).expect("Could not base64 decode blueprint");

    // decompress with zlib deflate
    let mut bp_decoder = ZlibDecoder::new(&decoded[..]);
    let mut json_string = String::new();
    bp_decoder.read_to_string(&mut json_string)?;
    Ok(json_string)
}

fn serialize_blueprint(json: &str) -> io::Result<Container> {
    let ret: Container = serde_json::from_str(json).expect("Could not deserialize json");
    Ok(ret)
}

// Call decode and then serialize to bring a compressed string to a Rust struct
pub fn read_blueprint(bp: &str) -> io::Result<Container> {
    Ok(serialize_blueprint(&decode_blueprint(bp)?)?)
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::BufReader, path::Path};
    #[test]
    fn test_read_blueprint() {
        use super::*;
        // serialize the sample json, and compare it to our version

        let sample_bp_f =
            File::open(Path::new("./resource/balancer.txt")).expect("Could not open balancer.txt");
        let mut bp_reader = BufReader::new(sample_bp_f);
        let mut bp_string = String::new();
        bp_reader
            .read_to_string(&mut bp_string)
            .expect("Could not read balancer.txt");

        let decoded_target_file = File::open(Path::new("./resource/balancer.json"))
            .expect("Could not open balancer.json");
        let mut json_reader = BufReader::new(decoded_target_file);
        let mut json_string = String::new();
        json_reader
            .read_to_string(&mut json_string)
            .expect("Could not read balancer.json");

        assert_eq!(
            read_blueprint(&bp_string).unwrap(),
            serialize_blueprint(&json_string).unwrap()
        )
    }
}
