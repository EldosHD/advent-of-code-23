use regex::Regex;
use std::fs::read_to_string;

#[derive(Debug)]
struct Boat {
    speed: u32,    // in mm/ms
    distance: u32, // in mm
}

#[derive(Debug)]
struct Race {
    time: u32,     // in ms
    distance: u32, // in mm
    ways_to_win: u32,
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}

fn main() {
    let lines = read_lines("input1.txt");

    let whitespace_re = Regex::new(r"[ ]+").unwrap();
    let times: Vec<u32> = whitespace_re
        .split(lines[0].split(":").collect::<Vec<&str>>()[1].trim())
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    let distances: Vec<u32> = whitespace_re
        .split(lines[1].split(":").collect::<Vec<&str>>()[1].trim())
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    let mut races: Vec<Race> = vec![];

    for i in 0..(times.len()) {
        races.push(Race {
            time: times[i],
            distance: distances[i],
            ways_to_win: 0,
        })
    }

    let mut product = 1;
    for mut r in races {
        let mut b = Boat {
            speed: 0,
            distance: 0,
        };

        for t in 0..r.time + 1 {
            b.speed = t;
            b.distance = b.speed * (r.time - t);
            if b.distance > r.distance {
                r.ways_to_win += 1;
            }
        }

        product *= r.ways_to_win;
    }

    println!("product {}", product);
}
