#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
use std::fs::File;
use std::io::{self, Read, Write};

fn main() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    for arg in std::env::args().skip(1) {
        let mut file = match File::open(&arg) {
            Err(why) => panic!("couldn't open {}: {}", arg, why),
            Ok(file) => file,
        };

        let mut contents: Vec<u8> = Vec::new();
        if file.read_to_end(&mut contents).is_err() {
            panic!("couldn't read from: {}", arg);
        }

        if stdout.write(&contents).is_err() {
            panic!("couldn't write to: {}", arg);
        }

    }
}
