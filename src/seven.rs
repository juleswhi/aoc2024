use std::fs;

enum Operation {
    Addition,
    Mulitplication
}

pub fn run() {
    let binding = fs::read_to_string("inputs/seven.txt").unwrap();
    let mut file = binding.split('\n').collect::<Vec<&str>>();
    file.pop();

    for line in file {
        let (sum, vals) = line.split_once(": ").unwrap();

        println!("{sum}");
    }
}
