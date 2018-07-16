use base64::{decode, encode};
use flate2::{read::ZlibDecoder, write::ZlibEncoder, Compression};
use serde_json;
use std::{
    fmt, io::{self, prelude::*},
};
use types::*;

// Grid type to coerce the entity list into
// TODO should this move over into types.rs?  'tis a type
// you've really gotta get a hold on your project structures, you do something slightly different each time
// here, types should probably be blueprint.rs and this should be like, render.rs or something
// move al thedecode/deserialze/read fns over there too

// NOTE: The Display impl on Grid isnt supposed to be the grand finale, ehre
// I haven't done anything abotu shapes and sizes yet, which is pretty crucial
// TODO separate cell type, with its own display, that comes out as a square

#[derive(Debug)]
pub struct Grid {
    // A 2D grid of cells, each of which can hold multiple entities
    pub cells: Vec<Vec<Vec<Entity>>>,
}

impl Grid {
    // TODO real error
    pub fn from(c: Container) -> Result<Self, String> {
        let bp = c.blueprint;
        let size = bp.size();
        let entities = bp.entities;
        let mut cells = vec![vec![vec![]; size]; size];
        for e in &entities {
            // replace the proper cell with the entity
            let pos = &e.position;
            // get the top left corner oriented coords from the center-oriented coords
            let (grid_x, grid_y) = pos.grid_coords(size); // this has f64 coords, truncate here
                                                          // you have a problem with overlaps - maybe store a Vec
            cells[grid_x][grid_y].push(e.clone());
        }
        Ok(Grid { cells })
    }

    pub fn max_cell_len(&self) -> usize {
        let mut max = 0;
        for line in &self.cells {
            for cell in line.iter() {
                let cell_len = cell.len();
                if cell_len > max {
                    max = cell_len;
                }
            }
        }
        max
    }

    pub fn max_line_len(&self) -> usize {
        let mut max = 0;
        for line in &self.cells {
            let mut line_max_len = line.len();
            for cell in line.iter() {
                let cell_len = cell.len();
                if cell_len > line_max_len {
                    line_max_len = cell_len;
                }
            }
            if line_max_len > max {
                max = line_max_len
            }
        }
        max
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // get the max lengths first
        let max_cell_len = self.max_cell_len() * ENTITY_LEN;
        let max_line_len = self.max_line_len() * max_cell_len;
        let mut ret = String::new();
        for line in &self.cells {
            let mut line_string = String::from("|");
            let line_padding_diff = ((max_line_len - line.len() * max_cell_len) / 2) + 1;
            for _ in 0..line_padding_diff {
                line_string.push_str(" ");
            }
            for cell in line.iter() {
                let mut cell_string = String::from(":c:");

                let cell_padding_diff = ((max_cell_len - cell.len() * ENTITY_LEN) / 2) + 1;
                for _ in 0..cell_padding_diff {
                    cell_string.push_str(" ");
                }
                for entity in cell.iter() {
                    // ensure its exactly ENTITY_LEN, no more, no less
                    let mut e = format!("{:1$}", entity, ENTITY_LEN);
                    e.truncate(ENTITY_LEN);
                    cell_string.push_str(&format!("{:1$}", e, ENTITY_LEN));
                }
                for _ in 0..cell_padding_diff {
                    cell_string.push_str(" ")
                }

                cell_string.push_str(":c:");
                line_string.push_str(&cell_string);
            }
            for _ in 0..line_padding_diff {
                line_string.push_str(" ");
            }
            line_string.push_str("|\n");
            ret.push_str(&line_string);
        }
        writeln!(f, "{}", ret)
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

fn deserialize_blueprint(json: &str) -> io::Result<Container> {
    let ret: Container = serde_json::from_str(json).expect("Could not deserialize json");
    Ok(ret)
}

// Call decode and then serialize to bring a compressed string to a Rust struct
pub fn read_blueprint(bp: &str) -> io::Result<Container> {
    Ok(deserialize_blueprint(&decode_blueprint(bp)?)?)
}

pub fn write_blueprint(c: &Container) -> io::Result<String> {
    // serialize to JSON
    let raw_json = serde_json::to_string(c)?;

    // compress with zlib
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write(&raw_json.as_bytes())?;
    let compressed = encoder.finish()?;

    // base64 encode
    let encoded = encode(&compressed);

    // version byte in front - 0 for factorio 0.15/0.16
    let ret = format!("0{}", encoded);

    Ok(ret)
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
            deserialize_blueprint(&json_string).unwrap()
        )
    }
    #[test]
    fn test_roundtrip_blueprint() {
        use super::*;

        let sample_bp_f =
            File::open(Path::new("./resource/balancer.txt")).expect("Could not open balancer.txt");
        let mut bp_reader = BufReader::new(sample_bp_f);
        let mut bp_string = String::new();
        bp_reader
            .read_to_string(&mut bp_string)
            .expect("Could not read balancer.txt");

        assert_eq!(
            bp_string,
            write_blueprint(&read_blueprint(&bp_string).unwrap()).unwrap()
        )
    }
}
