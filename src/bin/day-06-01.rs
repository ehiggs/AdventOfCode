use std::collections::BTreeSet;
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

fn find_marker(buffer: &[u8]) -> Option<usize> {
    let mut seen = buffer.iter().take(4).cloned().collect::<BTreeSet<u8>>();

    for (i, window) in buffer.windows(4).enumerate() {
        if is_unique(window) {
            let position = i + 4;
            println!(
                "Found mark at char {}: {}",
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
        println!("Usage: day-06-01 <input-file>");
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
    if let Some(position) = find_marker(&buffer) {
        println!("Marker is at {}", position);
    } else {
        println!("No marker found");
    }
}

#[cfg(test)]
mod test {
    use super::find_marker;
    #[test]
    fn test_example1() {
        assert_eq!(
            find_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb".as_bytes()),
            Some(7)
        );
    }

    #[test]
    fn test_example2() {
        assert_eq!(
            find_marker("bvwbjplbgvbhsrlpgdmjqwftvncz".as_bytes()),
            Some(5)
        );
    }
    #[test]
    fn test_example3() {
        assert_eq!(
            find_marker("nppdvjthqldpwncqszvftbrmjlhg".as_bytes()),
            Some(6)
        );
    }
    #[test]
    fn test_example4() {
        assert_eq!(
            find_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".as_bytes()),
            Some(10)
        );
    }
    #[test]
    fn test_example5() {
        assert_eq!(
            find_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".as_bytes()),
            Some(11)
        );
    }
}
