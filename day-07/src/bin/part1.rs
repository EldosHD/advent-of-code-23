use ::day_07::*;
use std::{fs::File, io::BufRead, io::BufReader};

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Clone, Copy)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPairs,
    Pair,
    HighCard,
    NoType,
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Game {
    hand: String,
    converted_hand: Vec<CardType>,
    hand_type: HandType,
    bid: i32,
    rank: i32,
}

fn main() {
    let lines = BufReader::new(File::open("input1.txt").unwrap()).lines();
    let mut games: Vec<Game> = vec![];

    for line in lines {
        let line = line.unwrap();
        let mut g = Game {
            hand: String::from(""),
            converted_hand: vec![],
            hand_type: HandType::NoType,
            bid: 0,
            rank: -1,
        };

        let line = line.split(" ").collect::<Vec<&str>>();

        let hand = line[0].to_string();
        let bid = line[1].parse().unwrap();

        if is_five_of_a_kind(&hand) {
            g.hand_type = HandType::FiveOfAKind;
        } else if is_four_of_a_kind(&hand) {
            g.hand_type = HandType::FourOfAKind;
        } else if is_full_house(&hand) {
            g.hand_type = HandType::FullHouse;
        } else if is_three_of_a_kind(&hand) {
            g.hand_type = HandType::ThreeOfAKind;
        } else if is_two_pair(&hand) {
            g.hand_type = HandType::TwoPairs;
        } else if is_one_pair(&hand) {
            g.hand_type = HandType::Pair;
        } else {
            g.hand_type = HandType::HighCard;
        }

        g.converted_hand = convert_hand(&hand);
        g.hand = hand;
        g.bid = bid;

        games.push(g);
    }

    games.sort_by_key(|a| (a.hand_type, a.converted_hand.clone()));
    games.reverse();

    for i in 0..games.len() {
        games[i].rank = (i + 1) as i32;
    }

    let mut total_winnings = 0;
    for g in games {
        total_winnings += g.rank * g.bid;
    }
    println!("total winnings: {}", total_winnings);
}
