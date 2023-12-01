use std::char;
use std::fs;

fn main() {
    let file_path = "data/part1_input.txt";
    let mut strings: Vec<String> = vec![];
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines = contents.lines();
    for line in lines {
        let char_vec: Vec<char> = line.chars().collect();
        let mut digits: Vec<char> = vec![];
        for c in char_vec {
            if c.is_numeric() {
                digits.push(c);
            }
        }
        let mut new_output = String::from("");
        let first = digits.first().copied().unwrap();
        let last = digits.last().copied().unwrap();
        new_output.push(first);
        new_output.push(last);
        strings.push(new_output);
    }

    let mut sum = 0;
    for s in strings {
        let number: i32 = s.parse().unwrap();
        sum += number;
    }
    println!("sum {}", sum)
}
