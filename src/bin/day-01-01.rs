use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: day-01-01 <input-file>");
        std::process::exit(1);
    }
    let input = &args[1];
    let file = File::open(input).unwrap();
    let reader = BufReader::new(file);
    let mut max_elf: usize = 0;
    let mut current_elf: usize = 0;
    for line in reader.lines() {
        let line = line.unwrap();

        if line == "" {
            if current_elf > max_elf {
                max_elf = current_elf;
            }
            current_elf = 0;
        } else {
            let fruit: usize = line.parse::<usize>().unwrap();
            current_elf += fruit;
        }
    }
    if current_elf > max_elf {
        max_elf = current_elf;
    }
    println!("Max elf has {}", max_elf);
}
