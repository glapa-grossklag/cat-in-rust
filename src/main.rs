use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

fn main() {
    let use_stdin = "-";

    // Process args.
    let mut args: Vec<_> = std::env::args().skip(1).collect();
    if args.is_empty() {
        args.push(use_stdin.into());
    }

    // Read and write.
    let mut contents: Vec<u8> = Vec::new();
    for arg in args {
        contents.clear();

        let mut file: Box<dyn BufRead> = if arg == use_stdin {
            Box::new(BufReader::new(io::stdin()))
        } else {
            Box::new(BufReader::new(File::open(&arg).unwrap()))
        };

        if file.read_to_end(&mut contents).is_err() {
            panic!("couldn't read from: {}", arg);
        }

        let mut stdout = io::stdout();
        if stdout.write(&contents).is_err() {
            panic!("couldn't write to stdout");
        }
    }
}
