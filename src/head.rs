

use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;

const DEFAULT_NUMBER_LINES: usize = 10;

fn print_file(s: &String, n: usize){
    let f = File::open(s);
    if let Ok(f ) = f{
        let f = BufReader::new(f);
        for line in f.lines().take(n){
            println!("{}", line.unwrap());
        }
    }
}

fn main(){
    let number_lines: String = std::env::args().nth(1).unwrap_or(10.to_string());
    
    for file in std::env::args(){
        print_file(&file, number_lines.parse().expect("fuck off"));
    }
}
