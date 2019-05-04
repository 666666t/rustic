use std::collections::HashMap;
use std::{env, fs, process};

#[derive(Debug)]
struct Line {
    position: u32,
    command: u8,
    parameter: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 || args.len() > 2 {
        println!("Invalid Arguments.\n\nUsage: cargo run <path>");
        process::exit(1);
    }
    //Intended Arguments: Single path,
    //Should lead to text file containing BASIC code

    let path = &args[1];
    //File Path

    let mut program: Vec<Line> = Vec::new();
    load_program_from_file(&mut program, path);
    //Creates vector containing members of the Line struct,
    //Loads the program from the file to the vector, line-by-line

    println!("{:?}", program);
    run(&program);
}

fn load_program_from_file(program: &mut Vec<Line>, path: &str) {
    let file = match fs::read_to_string(path) {
        Ok(file) => file,
        Err(error) => {
            println!("Error while opening file: {}", error);
            process::exit(1);
        }
    };
    //Reads file at given path to a string,
    //Fails and exits if file is invalid.

    let byline: Vec<&str> = file.split('\n').collect();

    for i in byline.iter() {
        if i.is_empty() {
            continue;
        }
        let mut line_by_whitespace = i.splitn(3, ' ');
        program.push(Line {
            position: line_by_whitespace
                .next()
                .unwrap()
                .trim()
                .parse()
                .expect("Error reading line"),
            command: match_dictionary(line_by_whitespace.next().unwrap().trim()),
            parameter: String::from(line_by_whitespace.next().unwrap()),
        });
    }
}

fn match_dictionary(command: &str) -> u8 {
    let dictionary: HashMap<&str, u8> = [("print", 0), ("let", 1), ("goto", 2)]
        .iter()
        .cloned()
        .collect();

    match dictionary.get(command.to_ascii_lowercase().as_str()) {
        Some(n) => return *n,
        None => {
            println!("Invalid Command Found");
            process::exit(1);
        }
    }
}

fn run(program: &Vec<Line>) {
    let mut variable_map: HashMap<&str, u32> = HashMap::new();

    for i in program.iter() {
        match i.command {
            0 => {
                 
            }
            1 => {
                let mut paramset: Vec<&str> = i.parameter.splitn(2, ' ').collect();
                let varname = paramset.remove(0).trim();
                paramset = paramset[0].split_whitespace().collect();

                for i in 0..paramset.len() {
                   
                }
                //First iteration, checks for alphabetic strings and compares them to hashmap
                //replaces string with matching u32, panics if no match

                for i in 0..paramset.len() {

                }
                //Second iteration, checks for * and / operators, takes the parsed output of the
                //left and right value, and replaces the left value with the result. the loop then
                //deletes the 2 no-longer-needed elements as they have been consumed.
                
                for i in 0..paramset.len() {

                }
                //Third iteration, performs the same actions as iteration 2 but with + and -.
                
                if paramset.len() > 2 {
                    panic!("Error while parsing LET statement on line {}", i.position);
                }
                //If vector somehow still has more than 2 elements ("=" and the result), panic


                let variable: u32 = match paramset[1].parse() {
                    Ok(n) => n,
                    Err(error) => panic!("Invalid result on let statement: {}",error),
                };
                //Parse final variable to u32
                
                variable_map.insert(varname, variable);
                //insert variable to hashmap, updating if already declared.
                println!("{:?}", variable_map);
            }
            2 => {

            }
            _ => {}
        }
    }
}
