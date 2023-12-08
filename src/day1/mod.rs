use std::{fs::File, io::{BufReader, BufRead}};

pub fn puzzle_a () {
    let path = "src/day1/input_example.txt";
    let file = File::open(path).expect("could not open file");
    let reader = BufReader::new(file);

    let mut string_vec: Vec<String> = Vec::new();

    for line in reader.lines() {
        let line = line.expect("unable to read line");
        string_vec.push(line);
        println!("{:#?}", string_vec);
    }
}