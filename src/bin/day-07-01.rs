use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct INode {
    name: String,
    size: usize,
    children: Vec<INode>,
}

impl INode {
    fn size_under_100k(&self) -> usize {
        let mut sum = if self.size <= 100_000 { self.size } else { 0 };
        for child in self.children.iter() {
            sum += child.size_under_100k();
        }
        sum
    }

    fn find_best_dir(&self, free_space: usize) -> usize {
        let mut best = self.size;
        for child in self.children.iter() {
            if child.size > free_space && child.size < best {
                best = child.find_best_dir(free_space);
            }
        }
        best
    }
}

fn parse_input<I, S>(dirname: &str, lines: &mut I) -> INode
where
    I: Iterator<Item = S>,
    S: AsRef<str>,
{
    let mut size = 0;
    let mut children = vec![];
    while let Some(line) = lines.next() {
        match line.as_ref().split_once(" ").unwrap() {
            ("$", cmd) => match cmd.split_once(" ") {
                Some(("cd", "..")) => {
                    return INode {
                        name: dirname.to_owned(),
                        children,
                        size,
                    }
                }
                Some(("cd", dir)) => {
                    let child = parse_input(dir, lines);
                    size += child.size;
                    children.push(child);
                }
                _ => {}
            },
            ("dir", _) => {}
            (file_size, _) => {
                size += file_size.parse::<usize>().unwrap();
            }
        }
    }
    return INode {
        name: dirname.to_owned(),
        children,
        size,
    };
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
    let mut lines = reader.lines().skip(1).map(|x| x.unwrap());
    let nodes = parse_input("/", &mut lines);
    println!("{:?}", nodes);
    println!(
        "Size of everything under 100,000: {}",
        nodes.size_under_100k(),
    );
    println!(
        "Size of smallest directory freeing enough space: {}",
        nodes.find_best_dir(30000000 - (70000000 - nodes.size)),
    );
}

#[cfg(test)]
mod test {
    use super::parse_input;
    #[test]
    fn test_example1() {
        let input = vec![
            "$ ls",
            "123000 a.txt",
            "456000 b.txt",
            "dir c",
            "$ cd c",
            "$ ls",
            "789 8.txt",
        ];
        let node = parse_input("/", &mut input.into_iter());
        println!("{:?}", node);
        assert_eq!(node.size_under_100k(), 789);
    }

    #[test]
    fn test_example2() {
        let input = vec![
            "$ ls",
            "dir a",
            "14848514 b.txt",
            "8504156 c.dat",
            "dir d",
            "$ cd a",
            "$ ls",
            "dir e",
            "29116 f",
            "2557 g",
            "62596 h.lst",
            "$ cd e",
            "$ ls",
            "584 i",
            "$ cd ..",
            "$ cd ..",
            "$ cd d",
            "$ ls",
            "4060174 j",
            "8033020 d.log",
            "5626152 d.ext",
            "7214296 k",
        ];
        let node = parse_input("/", &mut input.into_iter());
        println!("{:?}", node);
        assert_eq!(node.size_under_100k(), 95437);
    }
}
