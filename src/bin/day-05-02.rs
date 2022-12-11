use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, PartialEq, Eq)]
struct Command {
    source: usize,
    dest: usize,
    amt: usize,
}

fn parse_line(line: &str) -> Command {
    let tokens: Vec<&str> = line.split(' ').collect();
    assert_eq!(tokens[0], "move");
    let amt = tokens[1].parse::<usize>().unwrap();
    assert_eq!(tokens[2], "from");
    let source = tokens[3].parse::<usize>().unwrap() - 1;
    assert_eq!(tokens[4], "to");
    let dest = tokens[5].parse::<usize>().unwrap() - 1;

    Command { source, dest, amt }
}

fn apply_cmd(state: &mut Vec<Vec<char>>, cmd: Command) {
    let begin = state[cmd.source].len() - cmd.amt;
    let drain = state[cmd.source].drain(begin..);
    let mut buf = drain.collect::<Vec<char>>();
    state[cmd.dest].append(&mut buf);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: day-05-01 <input-file>");
        std::process::exit(1);
    }
    let input = &args[1];
    let file = File::open(input).unwrap();
    let reader = BufReader::new(file);

    //    [G]         [P]         [M]
    //    [V]     [M] [W] [S]     [Q]
    //    [N]     [N] [G] [H]     [T] [F]
    //    [J]     [W] [V] [Q] [W] [F] [P]
    //[C] [H]     [T] [T] [G] [B] [Z] [B]
    //[S] [W] [S] [L] [F] [B] [P] [C] [H]
    //[G] [M] [Q] [S] [Z] [T] [J] [D] [S]
    //[B] [T] [M] [B] [J] [C] [T] [G] [N]
    // 1   2   3   4   5   6   7   8   9
    let mut state = vec![
        vec!['B', 'G', 'S', 'C'],
        vec!['T', 'M', 'W', 'H', 'J', 'N', 'V', 'G'],
        vec!['M', 'Q', 'S'],
        vec!['B', 'S', 'L', 'T', 'W', 'N', 'N'],
        vec!['J', 'Z', 'F', 'T', 'V', 'G', 'W', 'P'],
        vec!['C', 'T', 'B', 'G', 'Q', 'H', 'S'],
        vec!['T', 'J', 'P', 'B', 'W'],
        vec!['G', 'D', 'C', 'Z', 'F', 'T', 'Q', 'M'],
        vec!['N', 'S', 'H', 'B', 'P', 'F'],
    ];

    let mut completed_header = false;
    for line in reader.lines() {
        let line = line.unwrap();
        // skip the header as we will hard code the originating state.
        if !completed_header {
            if line == "" {
                completed_header = true;
            }
            continue;
        }
        let cmd = parse_line(&line);
        apply_cmd(&mut state, cmd);
    }
    println!("Final state: {:?}", state);
    println!(
        "Top of each stack: {:?}",
        state.iter().map(|v| v.last().unwrap()).collect::<Vec<_>>()
    );
}

#[cfg(test)]
mod test {
    use super::{parse_line, Command};
    #[test]
    fn test_parse_line() {
        assert_eq!(
            parse_line("move 2 from 2 to 9"),
            Command {
                source: 1,
                dest: 8,
                amt: 2
            }
        );
    }
}
