use std::{fs, env};
fn  count_words(buf: &String) -> usize{
    let words: Vec<&str> =buf.split_ascii_whitespace().collect();
    words.len()
}
fn  count_lines(buf: &String) -> usize{
    let lines: Vec<&str> =buf.lines().collect();
    lines.len()
}
fn read_file(s: &String){
    let res = fs::read_to_string(s);
    if let Err(err) =  res{
        eprintln!("Could not read {} because of {}", s, err.to_string());
        return;
    }
    let res = res.unwrap();
    println!("{} {} {}", count_lines(&res), count_words(&res),  res.len());

}
fn main(){
    let args: Vec<String> = env::args().collect();
    let arg = args.get(1).unwrap_or_else(|| std::process::exit(1));

    read_file( &arg);

}
