use std::{env,fs,process};

#[derive(Debug)]
struct Line {
    position: i32,
    command: i8,
    parameter: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 || args.len() > 2 {
        println!("Invalid Arguments.\n\nUsage: cargo run <path>");
        process::exit(1);
    }
    let path = &args[1];
    let mut program: Vec<Line> = Vec::new();
    load_program_from_file(&mut program, path);
}

fn load_program_from_file(program: &mut Vec<Line>, path: &str) {
    let file = match fs::read_to_string(path) {
        Ok(file) => file,
        Err(error) => {
            println!("Error while opening file: {}",error);
            process::exit(1);
        }
    };
    let mut byline: Vec<&str> = file.split('\n').collect();
    if byline[byline.len() - 1] == "" {
        byline.pop();
    }
    println!("{:?}",byline);

}
