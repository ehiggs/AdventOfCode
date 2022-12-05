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
struct Group<'a> {
    first: &'a str,
    second: &'a str,
    third: &'a str,
}

impl<'a> Group<'a> {
    fn duplicated(&self) -> Option<char> {
        // whew n^2 loop!
        for item_1 in self.first.chars() {
            for item_2 in self.second.chars() {
                if item_1 == item_2 {
                    for item_3 in self.third.chars() {
                        if item_1 == item_3 {
                            return Some(item_1);
                        }
                    }
                }
            }
        }
        None
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: day-03-02 <input-file>");
        std::process::exit(1);
    }
    let input = &args[1];
    let file = File::open(input).unwrap();
    let mut reader = BufReader::new(file);
    let mut total_priority = 0;
    let mut keep_reading = true;
    let mut line_1 = String::new();
    let mut line_2 = String::new();
    let mut line_3 = String::new();
    while keep_reading {
        line_1.clear();
        line_2.clear();
        line_3.clear();
        keep_reading &= reader
            .read_line(&mut line_1)
            .map(|x| x != 0)
            .unwrap_or(false);
        keep_reading &= reader
            .read_line(&mut line_2)
            .map(|x| x != 0)
            .unwrap_or(false);
        keep_reading &= reader
            .read_line(&mut line_3)
            .map(|x| x != 0)
            .unwrap_or(false);
        if !keep_reading {
            break;
        }
        println!("{}", line_1);
        let group = Group {
            first: &line_1.trim(),
            second: &line_2.trim(),
            third: &line_3.trim(),
        };
        let duplicated = group.duplicated();
        let score = duplicated.map_or(0, priority);
        println!("{:?} duplicated: {:?}, score: {}", group, duplicated, score);
        total_priority += score;
    }
    println!("Total priority: {}", total_priority);
}

#[cfg(test)]
mod test {
    use super::{priority, Group};
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

    #[test]
    fn test_duplicated() {
        let group = Group {
            first: "BPDldDTDPZcggjcccTdNMbbMNSQNqqjtzMbrRb",
            second: "LvmWsfvssLGnQbQMRQqrSRnz",
            third: "WpvsVmmpmmfpfJGrHfVCHVvmcDgpDlZphgFgdhclhdgdBlgF",
        };
        assert_eq!(group.duplicated(), Some('r'));
    }
}
