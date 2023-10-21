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
    let mut lines = input_file_contents.lines();

    while let Some(line) = lines.next() {

        let first_rucksack = line;
        let second_rucksack = lines.next().unwrap();
        let third_rucksack = lines.next().unwrap();

        // Create map for contents of first and second rucksack
        let mut first_rucksack_contents: HashMap<char, bool> = HashMap::new();
        let mut second_rucksack_contents: HashMap<char, bool> = HashMap::new();
        
        // Map all unique characters in first rucsack
        for c in first_rucksack.chars() {
            if !first_rucksack_contents.contains_key(&c) {
                first_rucksack_contents.insert(c, true);
            }
        }

        // For second map, only add characters that are also present in first rucksack
        for c in second_rucksack.chars() {
            if first_rucksack_contents.contains_key(&c) && !second_rucksack_contents.contains_key(&c) {
                second_rucksack_contents.insert(c, true);
            }
        }
        
        // Knowing that there is only one character in all three, rucksacks, if key is found in second (derived) map
        // then that is common to all 3 rucksacks. Calculate character priority, sum, and break loop.
        for c in third_rucksack.chars() {
            if second_rucksack_contents.contains_key(&c) {
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