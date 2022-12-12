use std::collections::BTreeSet;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_move(line: &str) -> (i32, i32) {
    match line.split_once(' ').unwrap() {
        ("R", x) => (x.parse::<i32>().unwrap(), 0),
        ("L", x) => (-1 * x.parse::<i32>().unwrap(), 0),
        ("U", x) => (0, x.parse::<i32>().unwrap()),
        ("D", x) => (0, -1 * x.parse::<i32>().unwrap()),
        (_, _) => unimplemented!(),
    }
}

fn too_far(head: (i32, i32), tail: (i32, i32)) -> bool {
    (tail.0 - head.0).abs() > 1 || (tail.1 - head.1).abs() > 1
}

fn run<I, S>(lines: I) -> usize
where
    I: Iterator<Item = S>,
    S: AsRef<str>,
{
    let mut head_pos = (0, 0);
    let mut tail_pos = (0, 0);
    let mut tail_history: BTreeSet<(i32, i32)> = BTreeSet::new();
    tail_history.insert((0, 0));
    for line in lines {
        let head_move = parse_move(line.as_ref());
        for _ in 0..head_move.0.abs() {
            let coef = if head_move.0 > 0 { 1 } else { -1 };
            head_pos.0 += coef;
            if too_far(head_pos, tail_pos) {
                if head_pos.1 > tail_pos.1 {
                    tail_pos.1 += 1
                } else if head_pos.1 < tail_pos.1 {
                    tail_pos.1 -= 1
                }
                tail_pos.0 += coef;
                tail_history.insert(tail_pos);
            }
        }
        for _ in 0..head_move.1.abs() {
            let coef = if head_move.1 > 0 { 1 } else { -1 };
            head_pos.1 += coef;
            if too_far(head_pos, tail_pos) {
                if head_pos.0 > tail_pos.0 {
                    tail_pos.0 += 1
                } else if head_pos.0 < tail_pos.0 {
                    tail_pos.0 -= 1
                }
                tail_pos.1 += coef;
                tail_history.insert(tail_pos);
            }
        }
        /*
        println!(
            "Tail History state after move {:?} -> {:?}: {:?}",
            head_move, tail_pos, tail_history
        );
        */
    }
    tail_history.len()
}
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: day-06-01 <input-file>");
        std::process::exit(1);
    }
    let input = &args[1];
    let file = File::open(input).unwrap();
    let reader = BufReader::new(file);
    let tail_history_len = run(reader.lines().map(|x| x.unwrap()));

    println!("Tail has been to {} positions", tail_history_len);
}

#[cfg(test)]
mod test {
    use super::{run, too_far};
    #[test]
    fn test_distance() {
        assert!(!too_far((0, 0), (0, 0)));
        assert!(!too_far((1, 0), (0, 0)));
        assert!(!too_far((1, 1), (0, 0)));
        assert!(!too_far((0, 1), (0, 0)));
        assert!(!too_far((0, -1), (0, 0)));
        assert!(!too_far((-1, -1), (0, 0)));
        assert!(!too_far((-1, 0), (0, 0)));
        assert!(too_far((2, 0), (0, 0)));
        assert!(too_far((0, 0), (0, 2)));
    }
    #[test]
    fn test_example1() {
        let input = vec!["R 4", "U 4", "L 3", "D 1", "R 4", "D 1", "L 5", "R 2"];
        assert_eq!(run(input.iter()), 13);
    }
}
