use crate::{Solution, SolutionPair};
use std::collections::HashMap;
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

const MAX_PART_1: u32 = 100_000;
const TOTAL_SPACE: u32 = 70_000_000;
const REQUIRED_SPACE: u32 = 30_000_000;

struct FileData<'a> {
    path: Vec<&'a str>, // Doesn't include the name of the file, only the dir
    size: u32,
}

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day07.txt").unwrap();
    let files = parse(&input);

    let dir_sizes = get_dir_sizes(&files);
    let sol1: u32 = dir_sizes.values().copied().filter(|&x| x <= MAX_PART_1).sum();

    let min_size_delete = REQUIRED_SPACE - (TOTAL_SPACE - dir_sizes["/"]);
    let sol2: u32 = dir_sizes.values().copied().filter(|&x| x >= min_size_delete).min().unwrap();

    (Solution::from(sol1), Solution::from(sol2))
}

fn get_dir_sizes(files: &[FileData]) -> HashMap<String, u32> {
    let mut map = HashMap::new();

    for FileData { path, size } in files {
        for i in 1..=path.len() {
            let full_path = path[..i].join("/");
            let sum = map.entry(full_path).or_insert(0);
            *sum += size;
        }
    }

    map
}

fn parse(input: &str) -> Vec<FileData> {
    let mut cwd = vec![];
    let mut files = vec![];

    for line in input.lines() {
        // Remove the shell prefix if it's there and process the first word
        let spl: Vec<&str> = line.trim_start_matches("$ ").split(' ').collect();

        match spl[0] { 
            "cd" => { // Go into the specified directory, or one up if it's ..
                match spl[1] {
                    ".." => cwd.truncate(cwd.len() - 1),
                    x => cwd.push(x),
                };
            },
            s if s.as_bytes()[0].is_ascii_digit() => { // The line starts with a size, add the file
                files.push(FileData{ path: cwd.clone(), size: s.parse().unwrap() })
            },
            _ => {} // We can ignore everything else
        };
    }

    files
}