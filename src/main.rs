use std::env;
use std::process;
use std::fs;
use std::collections::HashMap;

fn main() {

    let args: Vec<String> = env::args().collect();

    let config: Config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    let input_file_contents = fs::read_to_string(config.file_path).unwrap_or_else(|err| {
        eprintln!("Problem reading input file: {err}");
        process::exit(1);
    });

    let mut priority_sum: u32 = 0;

    for line in input_file_contents.lines() {

        let line_middle: usize = (line.len() / 2);
        let (first_compartment, second_compartment) = line.split_at(line_middle);
        let mut first_compartment_contents: HashMap<char, u32> = HashMap::new();
        
        for c in first_compartment.chars() {
            first_compartment_contents.insert(c, calculate_priority(&c));
        }

        for c in second_compartment.chars() {
            if first_compartment_contents.contains_key(&c) {
                priority_sum += calculate_priority(&c);
                break;
            }
        }
    }

    println!("Priority sum: {}", priority_sum);
}

fn calculate_priority(character: &char) -> u32 {

    let c: char = character.clone();
    let char_ascii_val: u32 = c as u32;
    let mut char_priority: u32 = 0;

    // Get priority of character by ascii code and relevant offset
    // a - z priority 1 - 26
    // A - Z priority 27 - 52
    if char_ascii_val < 97 {
        char_priority = char_ascii_val - 38;
    } else {
        char_priority = char_ascii_val - 96;
    }

    char_priority
}

struct Config {
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {

        if args.len() < 2 {
            return Err("not enough arguments");
        }

        let file_path = args[1].clone();

        Ok(Config { file_path: file_path})
    }
}