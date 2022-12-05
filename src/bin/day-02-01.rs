use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
enum Move {
    Rock,
    Paper,
    Scissor,
}
#[derive(Debug)]
struct Game {
    my_move: Move,
    opponent_move: Move,
}

fn parse_game(line: &str) -> Game {
    let opponent_move = match line.chars().nth(0) {
        Some('A') => Move::Rock,
        Some('B') => Move::Paper,
        Some('C') => Move::Scissor,
        _ => unimplemented!(),
    };
    let my_move = match line.chars().nth(2) {
        Some('X') => Move::Rock,
        Some('Y') => Move::Paper,
        Some('Z') => Move::Scissor,
        _ => unimplemented!(),
    };
    Game {
        my_move,
        opponent_move,
    }
}

impl Game {
    /// 0 points loss; 3 points draw, 6 points win
    /// 1 point rock, 2 points paper, 3 points scissors
    fn score(&self) -> usize {
        match self {
            Game {
                my_move: Move::Rock,
                opponent_move: Move::Rock,
            } => 1 + 3,
            Game {
                my_move: Move::Paper,
                opponent_move: Move::Rock,
            } => 2 + 6,
            Game {
                my_move: Move::Scissor,
                opponent_move: Move::Rock,
            } => 3 + 0,
            Game {
                my_move: Move::Rock,
                opponent_move: Move::Paper,
            } => 1 + 0,
            Game {
                my_move: Move::Paper,
                opponent_move: Move::Paper,
            } => 2 + 3,
            Game {
                my_move: Move::Scissor,
                opponent_move: Move::Paper,
            } => 3 + 6,
            Game {
                my_move: Move::Rock,
                opponent_move: Move::Scissor,
            } => 1 + 6,
            Game {
                my_move: Move::Paper,
                opponent_move: Move::Scissor,
            } => 2 + 0,
            Game {
                my_move: Move::Scissor,
                opponent_move: Move::Scissor,
            } => 3 + 3,
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: day-01 <input-file>");
        std::process::exit(1);
    }
    let input = &args[1];
    let file = File::open(input).unwrap();
    let reader = BufReader::new(file);

    let mut total_score = 0;
    for (i, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let game = parse_game(&line);
        let score = game.score();
        println!("Game {} {:?} score: {}", i, game, score);
        total_score += score;
    }
    println!("Total score is {}", total_score);
}
