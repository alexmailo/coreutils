use std::env;
fn main(){
    let args: Vec<String> = env::args().collect();
    let default_message = String::from("y");
    let arg = args.get(1).unwrap_or(&default_message);
    
    loop {
        println!("{}", arg);
    }

}
