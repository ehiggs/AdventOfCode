use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn run<I, S>(lines: I, cycles: &Vec<i32>) -> i32
where
    I: Iterator<Item = S>,
    S: AsRef<str>,
{
    let mut cycle = 1;
    let mut register: i32 = 1;
    let mut power: i32 = 0;
    for line in lines {
        match line.as_ref() {
            "noop" => {
                cycle += 1;
                if cycles.contains(&cycle) {
                    power += cycle * register;
                    println!(
                        "Power incremented for cycle {} with register being {}",
                        cycle, register
                    );
                }
            }
            cmd => match cmd.split_once(' ').unwrap() {
                ("addx", val) => {
                    cycle += 1;
                    if cycles.contains(&cycle) {
                        power += cycle * register;
                        println!(
                            "Power incremented for cycle {} with register being {}",
                            cycle, register
                        );
                    }
                    cycle += 1;
                    register += val.parse::<i32>().unwrap();
                    if cycles.contains(&cycle) {
                        power += cycle * register;
                        println!(
                            "Power incremented for cycle {} with register being {}",
                            cycle, register
                        );
                    }
                }
                _ => unimplemented!(),
            },
        }
    }
    power
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
    let result = run(
        reader.lines().map(|x| x.unwrap()),
        &vec![20, 60, 100, 140, 180, 220],
    );
    println!("Result is {}", result);
}

#[cfg(test)]
mod test {
    use super::run;
    #[test]
    fn test_input_small() {
        let input = vec!["noop", "addx 3", "addx -5"];
        assert_eq!(run(input.iter(), &vec![0]), 0);
        assert_eq!(run(input.iter(), &vec![3]), 3);
    }
    #[test]
    fn test_input_long() {
        let input = vec![
            "addx 15", "addx -11", "addx 6", "addx -3", "addx 5", "addx -1", "addx -8", "addx 13",
            "addx 4", "noop", "addx -1", "addx 5", "addx -1", "addx 5", "addx -1", "addx 5",
            "addx -1", "addx 5", "addx -1", "addx -35", "addx 1", "addx 24", "addx -19", "addx 1",
            "addx 16", "addx -11", "noop", "noop", "addx 21", "addx -15", "noop", "noop",
            "addx -3", "addx 9", "addx 1", "addx -3", "addx 8", "addx 1", "addx 5", "noop", "noop",
            "noop", "noop", "noop", "addx -36", "noop", "addx 1", "addx 7", "noop", "noop", "noop",
            "addx 2", "addx 6", "noop", "noop", "noop", "noop", "noop", "addx 1", "noop", "noop",
            "addx 7", "addx 1", "noop", "addx -13", "addx 13", "addx 7", "noop", "addx 1",
            "addx -33", "noop", "noop", "noop", "addx 2", "noop", "noop", "noop", "addx 8", "noop",
            "addx -1", "addx 2", "addx 1", "noop", "addx 17", "addx -9", "addx 1", "addx 1",
            "addx -3", "addx 11", "noop", "noop", "addx 1", "noop", "addx 1", "noop", "noop",
            "addx -13", "addx -19", "addx 1", "addx 3", "addx 26", "addx -30", "addx 12",
            "addx -1", "addx 3", "addx 1", "noop", "noop", "noop", "addx -9", "addx 18", "addx 1",
            "addx 2", "noop", "noop", "addx 9", "noop", "noop", "noop", "addx -1", "addx 2",
            "addx -37", "addx 1", "addx 3", "noop", "addx 15", "addx -21", "addx 22", "addx -6",
            "addx 1", "noop", "addx 2", "addx 1", "noop", "addx -10", "noop", "noop", "addx 20",
            "addx 1", "addx 2", "addx 2", "addx -6", "addx -11", "noop", "noop", "noop",
        ];
        assert_eq!(run(input.iter(), &vec![20, 60, 100, 140, 180, 220]), 13140)
    }
}
