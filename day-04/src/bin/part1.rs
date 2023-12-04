use regex::Regex;
use std::fs::read_to_string;

#[derive(Debug)]
struct Card {
    id: u32,
    winning_numbers: Vec<u32>,
    card_numbers: Vec<u32>,
    points: u32,
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}
fn main() {
    let mut cards: Vec<Card> = vec![];
    let lines = read_lines("input1.txt");

    for line in lines {
        let mut card = Card {
            id: 0,
            winning_numbers: vec![],
            card_numbers: vec![],
            points: 0,
        };

        let s = line.split(":").collect::<Vec<&str>>();

        let re1 = Regex::new(r"(Card[ ]+)").unwrap();
        card.id = re1.replace_all(s[0], "").parse().unwrap();

        let numberset = s[1].split("|").collect::<Vec<&str>>();
        let winning_numbers_str = numberset[0].trim();
        let card_numbers_str = numberset[1].trim();

        let re2 = Regex::new(r"([0123456789]+)").expect("Invalid regex");

        for n in re2.find_iter(winning_numbers_str) {
            card.winning_numbers.push(n.as_str().parse().unwrap());
        }

        for n in re2.find_iter(card_numbers_str) {
            card.card_numbers.push(n.as_str().parse().unwrap());
        }

        cards.push(card);
    }

    for c in cards.iter_mut() {
        for n in &c.card_numbers {
            if c.winning_numbers.contains(&n) {
                if c.points == 0 {
                    c.points = 1;
                } else {
                    c.points *= 2;
                }
            }
        }
    }

    let mut sum = 0;
    for c in cards {
        sum += c.points;
    }

    println!("sum: {}", sum);
}
