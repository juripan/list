use colored::Colorize;
use std::fs::{read_dir, ReadDir};
use std::fs::metadata;
use std::env::args;


fn crop_letters(s: &str) -> &str {
    for (a, ch) in s.char_indices(){
        if ch == '\\'{
            return &s[a+1..];  
        } 
    }
    return &s[2..];
}


fn main() {
    const CURRENT_DIR: &str = "./";
    let args: Vec<String> = args().collect();
    let argc: usize = args.len();
    let dir_content: ReadDir;

    if argc > 2 {
        panic!("Too many arguments");
    } else if argc == 1 {
        dir_content = read_dir(CURRENT_DIR).unwrap();
    } else {
        dir_content = read_dir(&args[1]).expect("Expected a valid directory");
    }

    for path in dir_content{
        let dir_str = path.unwrap().path(); //here just so there are no drop borrow rules broken
        let dir_str = dir_str.to_str().unwrap();
        
        if metadata(dir_str).unwrap().is_dir(){
            print!("| {} ", crop_letters(dir_str).blue());
        }
        else {
            print!("| {} ", crop_letters(dir_str));
        }
    }
    println!(); //adds endline
}
