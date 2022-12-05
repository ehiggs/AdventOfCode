use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: day-01-02 <input-file>");
        std::process::exit(1);
    }
    let input = &args[1];
    let file = File::open(input).unwrap();
    let reader = BufReader::new(file);
    let mut max_elves: [usize; 3] = [0, 0, 0];
    let mut current_elf: usize = 0;
    for line in reader.lines() {
        let line = line.unwrap();

        if line == "" {
            register_elf(&mut max_elves, current_elf);
            current_elf = 0;
        } else {
            let fruit: usize = line.parse::<usize>().unwrap();
            current_elf += fruit;
        }
    }
    register_elf(&mut max_elves, current_elf);
    let sum = max_elves.iter().sum::<usize>();
    println!("Max elf has {:?} with sum {}", max_elves, sum);
}

fn register_elf(elves: &mut [usize; 3], elf: usize) {
    for i in 0..2 {
        if elf > elves[i] {
            elves[i] = elf;
            elves.sort();
            println!("{:?}", elves);
            return;
        }
    }
}
