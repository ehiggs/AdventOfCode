use std::cmp::Ordering;
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
fn next_move(head: (i32, i32), tail: (i32, i32)) -> (i32, i32) {
    let xcoef = match head.0.cmp(&tail.0) {
        Ordering::Greater => 1,
        Ordering::Less => -1,
        Ordering::Equal => 0,
    };
    let ycoef = match head.1.cmp(&tail.1) {
        Ordering::Greater => 1,
        Ordering::Less => -1,
        Ordering::Equal => 0,
    };
    let mut new_tail = (0, 0);
    if too_far(head, tail) {
        if head.0 != tail.0 {
            new_tail.0 += xcoef;
        }
        if head.1 != tail.1 {
            new_tail.1 += ycoef
        }
    }
    new_tail
}

fn run<I, S>(lines: I) -> usize
where
    I: Iterator<Item = S>,
    S: AsRef<str>,
{
    let mut positions = vec![
        (0, 0), // head
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0), // tail
    ];
    let mut tail_history: BTreeSet<(i32, i32)> = BTreeSet::new();
    tail_history.insert((0, 0));
    for line in lines {
        let head_move = parse_move(line.as_ref());
        let xcoef = match head_move.0.cmp(&0) {
            Ordering::Greater => 1,
            Ordering::Less => -1,
            Ordering::Equal => 0,
        };
        let ycoef = match head_move.1.cmp(&0) {
            Ordering::Greater => 1,
            Ordering::Less => -1,
            Ordering::Equal => 0,
        };
        for _ in 0..(head_move.0.abs() + head_move.1.abs()) {
            positions[0].0 += xcoef;
            positions[0].1 += ycoef;
            for (i, j) in (0..positions.len() - 1).zip(1..positions.len()) {
                let tail_move = next_move(positions[i], positions[j]);
                positions[j].0 += tail_move.0;
                positions[j].1 += tail_move.1;
            }
            tail_history.insert(positions.last().unwrap().clone());
        }
        //println!("** Move {:?} -> {:?}", head_move, positions);
        //println!("-- Tail History state after move -- {:?}", tail_history);
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
    use super::{next_move, run, too_far};
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
    fn test_next_move() {
        assert_eq!(next_move((0, 0), (0, 0)), (0, 0));
        assert_eq!(next_move((2, 0), (0, 0)), (1, 0));
        assert_eq!(next_move((1, 0), (0, 0)), (0, 0));
        assert_eq!(next_move((1, 0), (0, 0)), (0, 0));
        assert_eq!(next_move((1, -2), (0, 0)), (1, -1));
        assert_eq!(next_move((5, 0), (0, 0)), (1, 0));
    }

    #[test]
    fn test_example_input_day9() {
        let input = vec!["R 5", "U 8", "L 8", "D 3", "R 17", "D 10", "L 25", "U 20"];
        assert_eq!(run(input.iter()), 36);
    }
}
