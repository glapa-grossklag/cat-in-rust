#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
use std::fs::File;
use std::io::{self, Write, BufRead, BufReader};


fn main() {
    let use_stdin: String = "-".to_owned();

    let mut contents: Vec<u8> = Vec::new();

    for arg in std::env::args().skip(1) {
        contents.clear();

        let mut file: Box<dyn BufRead> = if arg == "-" {
            Box::new(BufReader::new(io::stdin()))
        } else {
            Box::new(BufReader::new(File::open(&arg).unwrap()))
        };

        if file.read_to_end(&mut contents).is_err() {
            panic!("couldn't read from: {}", arg);
        }

        let mut stdout = io::stdout();
        if stdout.write(&contents).is_err() {
            panic!("couldn't write to: {}", arg);
        }

    }
}
