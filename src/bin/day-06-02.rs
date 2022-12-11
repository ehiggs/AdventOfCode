use std::env;
use std::fs::File;
use std::io::{BufReader, Read};

fn is_unique(window: &[u8]) -> bool {
    for i in 0..window.len() - 1 {
        for j in i + 1..window.len() {
            if window[i] == window[j] {
                return false;
            }
        }
    }
    return true;
}

fn find_message(buffer: &[u8]) -> Option<usize> {
    for (i, window) in buffer.windows(14).enumerate() {
        if is_unique(window) {
            let position = i + 14;
            println!(
                "Found message at char {}: {}",
                position,
                std::str::from_utf8(window).unwrap()
            );
            return Some(position);
        }
    }
    None
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: day-06-02 <input-file>");
        std::process::exit(1);
    }
    let input = &args[1];
    let file = File::open(input).unwrap();
    let mut reader = BufReader::new(file);
    let mut buffer = vec![];
    reader
        .read_to_end(&mut buffer)
        .expect("Could not read the file");
    let buffer = buffer;
    if let Some(position) = find_message(&buffer) {
        println!("Message is at {}", position);
    } else {
        println!("No message found");
    }
}

#[cfg(test)]
mod test {
    use super::find_message;
    #[test]
    fn test_example1() {
        assert_eq!(
            find_message("mjqjpqmgbljsphdztnvjfqwrcgsmlb".as_bytes()),
            Some(19)
        );
    }

    #[test]
    fn test_example2() {
        assert_eq!(
            find_message("bvwbjplbgvbhsrlpgdmjqwftvncz".as_bytes()),
            Some(23)
        );
    }
    #[test]
    fn test_example3() {
        assert_eq!(
            find_message("nppdvjthqldpwncqszvftbrmjlhg".as_bytes()),
            Some(23)
        );
    }
    #[test]
    fn test_example4() {
        assert_eq!(
            find_message("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".as_bytes()),
            Some(29)
        );
    }
    #[test]
    fn test_example5() {
        assert_eq!(
            find_message("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".as_bytes()),
            Some(26)
        );
    }
}
