
use std::fs::{File};
use std::io::{BufReader, BufRead};
fn print_file(s: &str) {
    let mut lines = BufReader::new(File::open(s).expect("File does not exist"))
        .lines()
        .map(|x| x.unwrap())
        .collect::<Vec<_>>();

    for (i,line) in lines.iter().enumerate(){
        println!("{} {}",i+ 1,  line);

    }
}

fn main() {
    for file in std::env::args().skip(1) {
        print_file(&file);
    }
}
