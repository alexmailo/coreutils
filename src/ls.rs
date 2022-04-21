fn print_files( directory: &String) {
    if let Ok(rd) = std::fs::read_dir(directory){
        for ent in rd {
            
            if let Ok(file) = ent{
                println!("{}", file.file_name().to_str().expect("failed to convert os str to str"));
                continue;
            }
        }
        return
    }
    println!("Could not read directory {}", directory);
}
fn main(){
    for file in std::env::args().skip(1){
        print_files(&file);
    }
}
