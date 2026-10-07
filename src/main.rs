use colored::Colorize;
use tail;
use std::fs::File;
use std::io::{BufReader, BufWriter};

fn main() {
    let text: &str = &"Hello, world!";
    println!("{}",text.blue());

    let filename = std::env::args().nth(1);
    let fd = File::open(filename).unwrap();
    let mut fd = BufReader::new(fd);
    let mut reader = BackwardsReader::new(10, &mut fd);

    let mut out = BufWriter::new(std::io::stdout());
    reader.read_all(&mut out);
}
