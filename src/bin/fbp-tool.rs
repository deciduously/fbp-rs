extern crate fbp_rs;

use fbp_rs::blueprint::{read_blueprint, Grid};
use std::{
    env, fs::File, io::{prelude::*, BufReader}, path::Path,
};

fn main() {
    let args: Vec<String> = env::args().collect();

    // If invoked with no args, use balancer.txt
    // Otherwise attempt to use the first arg
    let bp_string = if args.len() == 1 {
        println!("No argument given, using balancer.txt");
        let bp_file =
            File::open(Path::new("./resource/balancer.txt")).expect("Could not open balancer.txt");
        let mut bp_str_reader = BufReader::new(bp_file);
        let mut s = String::new();
        bp_str_reader
            .read_to_string(&mut s)
            .expect("Could not read balancer.txt");
        s
    } else {
        args[1].clone()
    };

    let parsed_bp = read_blueprint(&bp_string).unwrap();
    let grid = Grid::from(parsed_bp).unwrap();

    println!("fbp-tool\n{:#?}", grid);
}
