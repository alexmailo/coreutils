use std::fs::File;
use std::io::{BufRead, BufReader};
fn print_n_lines(s: &str, n: usize) {
  
    let mut lines = BufReader::new(File::open(s).expect("File does not exist"))
        .lines()
        .map(|x| x.unwrap())
        .collect::<Vec<_>>();
    lines.reverse();
    let it = lines.iter();


    for line in it.rev().take(n) {
        println!("{}", line);
    }
}

fn main() {
    for file in std::env::args().skip(1) {
        print_n_lines(&file, 10);
    }
}
