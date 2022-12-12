use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn print_sprite(message: &mut Vec<String>, position: usize, register: i32) {
    let line: usize = position / 40;
    let xpos = (position % 40) as i32;
    if xpos >= register - 1 && xpos <= register + 1 {
        message[line].push('#');
    } else {
        message[line].push('.');
    }
}
fn run<I, S>(lines: I) -> Vec<String>
where
    I: Iterator<Item = S>,
    S: AsRef<str>,
{
    let mut cycle: usize = 1;
    let mut register: i32 = 1;
    let mut message: Vec<String> = vec![
        "".to_owned(),
        "".to_owned(),
        "".to_owned(),
        "".to_owned(),
        "".to_owned(),
        "".to_owned(),
    ];
    for line in lines {
        print_sprite(&mut message, cycle - 1, register);
        match line.as_ref() {
            "noop" => {
                cycle += 1;
            }
            cmd => match cmd.split_once(' ').unwrap() {
                ("addx", val) => {
                    cycle += 1;
                    print_sprite(&mut message, cycle - 1, register);
                    cycle += 1;
                    register += val.parse::<i32>().unwrap();
                }
                _ => unimplemented!(),
            },
        }
    }
    message
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
    let result = run(reader.lines().map(|x| x.unwrap()));
    for line in result.iter() {
        println!("{}", line);
    }
}

#[cfg(test)]
mod test {
    use super::run;
    #[test]
    fn test_input_long_message() {
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
        let expected = vec![
            "##..##..##..##..##..##..##..##..##..##..",
            "###...###...###...###...###...###...###.",
            "####....####....####....####....####....",
            "#####.....#####.....#####.....#####.....",
            "######......######......######......####",
            "#######.......#######.......#######.....",
        ];
        assert_eq!(run(input.iter()), expected);
    }
}
