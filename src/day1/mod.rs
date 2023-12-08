use std::{fs::File, io::{BufReader, BufRead}, fmt::Error, usize};

pub fn puzzle_a () {
    let path: &str = "src/day1/input_real.txt";
    let file: File = File::open(path).expect("could not open file");
    let reader: BufReader<File> = BufReader::new(file);

    let mut string_vec: Vec<String> = Vec::new();

    for line in reader.lines() {
        let line: String = line.expect("unable to read line");
        string_vec.push(line);
    }

    let answer: Vec<u32> = string_vec.iter().map(|s| {
        let digit_string_vec = extract_digit_string(s).unwrap();
        let num_string_vec = create_num_string(&digit_string_vec);
        let num_vec = num_string_vec.parse::<u32>().unwrap();
        num_vec
    }).collect();

    let answer: u32 = answer.iter().sum();
    println!("{:#?}", answer);
}

fn extract_digit_string(single_string: &String) -> Result<Vec<String>, Error>  {

    let num_vec_str: Vec<&str> = single_string.matches(char::is_numeric).collect();
    let num_vec = num_vec_str.iter().map(|s| s.to_string()).collect();
    Ok(num_vec)
}

fn create_num_string(vec_of_nums: &Vec<String>) -> String {
    let size: usize = vec_of_nums.len();
    let last_idx = size - 1; 

    let mut number_string: String = String::new();
    number_string.push_str(vec_of_nums[0].as_str());
    number_string.push_str(vec_of_nums[last_idx].as_str());
    number_string
}

pub fn puzzle_b () {
    let path: &str = "src/day1/input_example_b.txt";
    let file: File = File::open(path).expect("could not open file");
    let reader: BufReader<File> = BufReader::new(file);

    let mut string_vec: Vec<String> = Vec::new();

    for line in reader.lines() {
        let line: String = line.expect("unable to read line");
        string_vec.push(line);
    }

    let answer: Vec<(String, usize)> = extract_spell_nums(&string_vec[3]).unwrap();
    println!("{:#?}", answer);
}

fn extract_spell_nums(single_string: &String) -> Result<Vec<(String, usize)>, Error> {
    let to_match_spelled = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine" ];
    let mut result: Vec<(String, usize)> = Vec::new();

    for number_in_text in to_match_spelled {
        // let index = single_string.find(number).unwrap();
        if let Some(index) = single_string.find(number_in_text) {
            result.push((number_in_text.to_string(), index))
        }
    }
    
    Ok(result)
}