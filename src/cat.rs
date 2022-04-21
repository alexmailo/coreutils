use std::fs;
fn print_file(s: &str){
    let res = fs::read_to_string(s);
    if let Err(err) =  res{
        eprintln!("Could not read {} because of {}", s, err.to_string());
        return;
    }
    println!("{}", res.unwrap());

}

fn main(){
    for file in std::env::args().skip(1){
        print_file(&file);
    }
}
