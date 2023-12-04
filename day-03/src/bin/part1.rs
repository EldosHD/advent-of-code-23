use std::{char, fs::read_to_string, ops::Add};

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}

fn neighbour_is_symbol(x_pos: usize, y_pos: usize, char_array: Vec<Vec<char>>) -> bool {
    let mut is_symbol = false;
    let numbers = "0123456789.".to_string();
    for y in (y_pos - 1)..(y_pos + 1) {
        for x in (x_pos - 1)..(x_pos + 1) {
            if !(numbers.contains(char_array[y][x])) {
                is_symbol = true;
            }
        }
    }
    is_symbol
}

fn main() {
    let lines = read_lines("input1.txt");
    let mut char_vec: Vec<Vec<char>> = vec![];

    for line in lines {
        let mut l: Vec<char> = vec![];
        for c in line.chars() {
            l.push(c);
        }
        char_vec.push(l);
    }

    let mut numbers_with_neighbours: Vec<String> = vec![];

    // body without edges
    for y in 1..(char_vec.len() - 2) {
        for x in 1..(char_vec[y].len() - 2) {
            if neighbour_is_symbol(x, y, char_vec.clone()) {
                let mut n = char_vec[y][x].to_string();
                let mut offset = 0;
                loop {
                    offset += 1;
                    if "0123456789".to_string().contains(char_vec[y][x - offset]) {
                        n.insert(0, char_vec[y][x - offset]);
                        println!("offset {},pushed {}", offset, char_vec[y][x - offset]);
                    } else {
                        break;
                    }
                }

                offset = 0;
                loop {
                    offset += 1;
                    if "0123456789".to_string().contains(char_vec[y][x + offset]) {
                        n.insert(0, char_vec[y][x - offset]);
                        println!("offset {},pushed {}", offset, char_vec[y][x - offset]);
                    } else {
                        break;
                    }
                }
                numbers_with_neighbours.push(n);
            }
        }
    }
}
