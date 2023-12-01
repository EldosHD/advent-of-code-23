use std::char;
use std::fs;

fn convert_to_number(mut s: String) -> String {
    s = s.replace("zero", "0");
    s = s.replace("one", "1");
    s = s.replace("two", "2");
    s = s.replace("three", "3");
    s = s.replace("four", "4");
    s = s.replace("five", "5");
    s = s.replace("six", "6");
    s = s.replace("seven", "7");
    s = s.replace("eigth", "8");
    s = s.replace("nine", "9");

    s
}

fn main() {
    let file_path = "data/part2_input.txt";
    let mut strings: Vec<String> = vec![];
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines = contents.lines();
    for l in lines {
        let line = &convert_to_number(l.to_string());
        println!("line: {}", line);
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
