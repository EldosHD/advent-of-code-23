use regex::Regex;
use std::fs::read_to_string;

#[derive(Debug)]
struct Boat {
    speed: u32,    // in mm/ms
    distance: u64, // in mm
}

#[derive(Debug)]
struct Race {
    time: u32,     // in ms
    distance: u64, // in mm
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
    let time: u32 = whitespace_re
        .replace_all(lines[0].split(":").collect::<Vec<&str>>()[1].trim(), "")
        .parse()
        .unwrap();
    let distance: u64 = whitespace_re
        .replace_all(lines[1].split(":").collect::<Vec<&str>>()[1].trim(), "")
        .parse()
        .unwrap();

    let mut races: Vec<Race> = vec![];
    println!("time: {}, distance {}", time, distance);

    races.push(Race {
        time: time,
        distance: distance,
        ways_to_win: 0,
    });

    let mut product = 1;
    for mut r in races {
        let mut b = Boat {
            speed: 0,
            distance: 0,
        };

        for t in 0..r.time + 1 {
            b.speed = t;
            let d = u64::from(b.speed) * (u64::from(r.time) - u64::from(t));
            println!("t: {} d: {}", t, d);
            b.distance = u64::from(d);
            if b.distance > r.distance {
                r.ways_to_win += 1;
            }
        }

        product *= r.ways_to_win;
    }

    println!("product {}", product);
}
