use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn priority(c: char) -> usize {
    match c {
        'a'..='z' => (c as usize) - ('a' as usize) + 1,
        'A'..='Z' => (c as usize) - ('A' as usize) + 27,
        _ => unimplemented!(),
    }
}

#[derive(Debug)]
struct Backpack<'a> {
    first: &'a str,
    second: &'a str,
}

impl<'a> Backpack<'a> {
    fn duplicated(&self) -> Option<char> {
        // whew n^2 loop!
        for item_1 in self.first.chars() {
            for item_2 in self.second.chars() {
                if item_1 == item_2 {
                    return Some(item_1);
                }
            }
        }
        None
    }
}

fn parse_line<'a>(line: &'a str) -> Backpack<'a> {
    let (first, second) = line.split_at(line.len() / 2);
    Backpack {
        first: first,
        second: second,
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: day-03-01 <input-file>");
        std::process::exit(1);
    }
    let input = &args[1];
    let file = File::open(input).unwrap();
    let reader = BufReader::new(file);
    let mut total_priority = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let backpack = parse_line(&line);
        let duplicated = backpack.duplicated();
        let score = duplicated.map_or(0, priority);
        println!(
            "{:?} duplicated: {:?}, score: {}",
            backpack, duplicated, score
        );
        total_priority += score;
    }
    println!("Total priority: {}", total_priority);
}

#[cfg(test)]
mod test {
    use super::priority;
    #[test]
    fn test_priority() {
        assert_eq!(priority('a'), 1);
        assert_eq!(priority('t'), 20);
        assert_eq!(priority('p'), 16);
        assert_eq!(priority('v'), 22);
        assert_eq!(priority('z'), 26);
        assert_eq!(priority('A'), 27);
        assert_eq!(priority('L'), 38);
        assert_eq!(priority('P'), 42);
        assert_eq!(priority('Z'), 52);
    }
}
