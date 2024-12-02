use std::{
    collections, fs,
    io::{self, BufRead},
};

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    assert_eq!(
        args.len(),
        3,
        "invalid command arguments (ex.: comm file1 file2)"
    );

    let mut first_file_lines = collections::HashSet::new();

    let mut file = fs::File::open(args[1].clone()).unwrap();
    let mut reader = io::BufReader::new(file);
    for line in reader.lines().flatten() {
        first_file_lines.insert(line.trim().to_owned());
    }

    file = fs::File::open(args[2].clone()).unwrap();
    reader = io::BufReader::new(file);
    for line in reader.lines().flatten() {
        let trimmed_line = line.trim();
        if first_file_lines.contains(trimmed_line) {
            println!("{}", first_file_lines.take(trimmed_line).unwrap());
        }
    }
}
