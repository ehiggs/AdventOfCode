use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Assignment {
    section_1: (usize, usize),
    section_2: (usize, usize),
}
impl Assignment {
    fn overlap(&self) -> bool {
        (self.section_1.0 >= self.section_2.0 && self.section_1.1 <= self.section_2.1)
            || (self.section_1.1 >= self.section_2.0 && self.section_1.1 <= self.section_2.1)
            || (self.section_2.0 >= self.section_1.0 && self.section_2.1 <= self.section_1.1)
            || (self.section_2.1 >= self.section_1.0 && self.section_2.1 <= self.section_1.1)
    }
}

fn parse_pair(input: &str) -> (usize, usize) {
    let pair: Vec<&str> = input.split('-').collect();
    (
        str::parse::<usize>(pair[0]).unwrap(),
        str::parse::<usize>(pair[1]).unwrap(),
    )
}
fn parse_line(line: &str) -> Assignment {
    let pairs: Vec<&str> = line.split(',').collect();
    let pair_1 = parse_pair(pairs[0]);
    let pair_2 = parse_pair(pairs[1]);

    Assignment {
        section_1: pair_1,
        section_2: pair_2,
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: day-04-02 <input-file>");
        std::process::exit(1);
    }
    let input = &args[1];
    let file = File::open(input).unwrap();
    let reader = BufReader::new(file);
    let mut total_pairs = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let assignment = parse_line(&line);
        if assignment.overlap() {
            println!("Overlaps found: {:?}", assignment);
            total_pairs += 1;
        }
    }
    println!("Total pairs: {}", total_pairs);
}

#[cfg(test)]
mod test {
    use super::Assignment;
    #[test]
    fn test_overlap() {
        let assignment = Assignment {
            section_1: (0, 10),
            section_2: (8, 12),
        };
        assert!(assignment.overlap());

        let assignment = Assignment {
            section_1: (0, 10),
            section_2: (11, 12),
        };
        assert!(!assignment.overlap());

        let assignment = Assignment {
            section_1: (5, 10),
            section_2: (1, 5),
        };
        assert!(assignment.overlap());
    }
}
