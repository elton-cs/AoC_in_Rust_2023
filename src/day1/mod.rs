use std::{fs::File, io::{BufReader, BufRead}, fmt::Error};

pub fn puzzle_a () {
    let path = "src/day1/input_example.txt";
    let file = File::open(path).expect("could not open file");
    let reader = BufReader::new(file);

    let mut string_vec: Vec<String> = Vec::new();

    for line in reader.lines() {
        let line = line.expect("unable to read line");
        string_vec.push(line);
    }
    println!("{:#?}", string_vec);
    
    let answer: Vec<Vec<String>>= string_vec.iter().map(|s| extract_nums(s).unwrap()).collect();
    println!("{:#?}", answer);

}

pub fn extract_nums(single_string: &String) -> Result<Vec<String>, Error>  {

    let num_vec_str: Vec<&str> = single_string.matches(char::is_numeric).collect();
    let num_vec = num_vec_str.iter().map(|s| s.to_string()).collect();

    Ok(num_vec)
}