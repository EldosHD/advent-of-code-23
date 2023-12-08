use std::collections::HashMap;

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Clone, Copy)]
pub enum CardType {
    A,
    K,
    Q,
    J,
    T,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
}

pub fn convert_hand(hand: &String) -> Vec<CardType> {
    let chars: Vec<char> = hand.chars().collect();
    let mut converted_hand: Vec<CardType> = vec![];

    for c in chars {
        if c == 'A' {
            converted_hand.push(CardType::A);
        }
        if c == 'K' {
            converted_hand.push(CardType::K);
        }
        if c == 'Q' {
            converted_hand.push(CardType::Q);
        }
        if c == 'J' {
            converted_hand.push(CardType::J);
        }
        if c == 'T' {
            converted_hand.push(CardType::T);
        }
        if c == '9' {
            converted_hand.push(CardType::Nine);
        }
        if c == '8' {
            converted_hand.push(CardType::Eight);
        }
        if c == '7' {
            converted_hand.push(CardType::Seven);
        }
        if c == '6' {
            converted_hand.push(CardType::Six);
        }
        if c == '5' {
            converted_hand.push(CardType::Five);
        }
        if c == '4' {
            converted_hand.push(CardType::Four);
        }
        if c == '3' {
            converted_hand.push(CardType::Three);
        }
        if c == '2' {
            converted_hand.push(CardType::Two);
        }
    }

    converted_hand
}

fn count_letters(chars: &Vec<char>) -> HashMap<char, i32> {
    let mut letters: HashMap<char, i32> = HashMap::new();

    for c in chars.to_owned() {
        letters
            .entry(c)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }

    letters
}

fn get_pair_count(letters: HashMap<char, i32>) -> i32 {
    let mut pair_count = 0;

    for (_ch, count) in letters {
        if count == 2 {
            pair_count += 1;
        }
    }

    pair_count
}

fn get_unique_chars(chars: &Vec<char>) -> Vec<char> {
    let mut unique_chars: Vec<char> = vec![];

    for c in chars {
        if !unique_chars.contains(&c) {
            unique_chars.push(c.to_owned());
        }
    }

    unique_chars
}

pub fn is_five_of_a_kind(hand: &String) -> bool {
    let chars: Vec<char> = hand.chars().collect();
    let mut result = true;
    let first_char = chars[0];

    for c in chars {
        if c != first_char {
            result = false;
        }
    }

    result
}

pub fn is_four_of_a_kind(hand: &String) -> bool {
    let chars: Vec<char> = hand.chars().collect();
    let mut result = false;
    let letters = count_letters(&chars);

    for (_ch, count) in letters.iter() {
        if count == &i32::from(4) {
            result = true;
        }
    }

    result
}

pub fn is_full_house(hand: &String) -> bool {
    let chars: Vec<char> = hand.chars().collect();
    let letters = count_letters(&chars);
    let mut has_three = false;
    let mut has_two = false;

    for (_ch, count) in letters.iter() {
        if count == &i32::from(3) {
            has_three = true;
        }
        if count == &i32::from(2) {
            has_two = true;
        }
    }

    has_three && has_two
}

pub fn is_three_of_a_kind(hand: &String) -> bool {
    let chars: Vec<char> = hand.chars().collect();
    let mut result = false;
    let letters = count_letters(&chars);

    for (_ch, count) in letters.iter() {
        if count == &i32::from(3) {
            result = true;
        }
    }

    result
}

pub fn is_two_pair(hand: &String) -> bool {
    let chars: Vec<char> = hand.chars().collect();
    let letters = count_letters(&chars);
    let pair_count = get_pair_count(letters);

    pair_count == 2
}

pub fn is_one_pair(hand: &String) -> bool {
    let chars: Vec<char> = hand.chars().collect();
    let letters = count_letters(&chars);
    let pair_count = get_pair_count(letters);

    pair_count == 1
}

fn get_input() -> Vec<String> {
    let input: Vec<String> = vec![
        "AAAAA".to_string(),
        "A2222".to_string(),
        "AA222".to_string(),
        "AAA34".to_string(),
        "22A33".to_string(),
        "123AA".to_string(),
        "123KA".to_string(),
    ];

    input
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_five_kind() {
        let input = get_input();

        for i in 0..(input.len()) {
            if i == 0 {
                assert_eq!(is_five_of_a_kind(&input[i]), true);
            } else {
                assert_eq!(is_five_of_a_kind(&input[i]), false);
            }
        }
    }

    #[test]
    fn test_four_kind() {
        let input = get_input();

        for i in 0..(input.len()) {
            if i == 1 {
                assert_eq!(is_four_of_a_kind(&input[i]), true);
            } else {
                assert_eq!(is_four_of_a_kind(&input[i]), false);
            }
        }
    }

    #[test]
    fn test_full_house() {
        let input = get_input();

        for i in 0..(input.len()) {
            if i == 2 {
                assert_eq!(is_full_house(&input[i]), true);
            } else {
                assert_eq!(is_full_house(&input[i]), false);
            }
        }
    }

    #[test]
    fn test_three_kind() {
        let input = get_input();

        for i in 0..(input.len()) {
            if i == 2 || i == 3 {
                assert_eq!(is_three_of_a_kind(&input[i]), true);
            } else {
                assert_eq!(is_three_of_a_kind(&input[i]), false);
            }
        }
    }

    #[test]
    fn test_two_pair() {
        let input = get_input();

        for i in 0..(input.len()) {
            if i == 4 {
                assert_eq!(is_two_pair(&input[i]), true);
            } else {
                assert_eq!(is_two_pair(&input[i]), false);
            }
        }
    }

    #[test]
    fn test_one_pair() {
        let input = get_input();

        for i in 0..(input.len()) {
            if i == 2 || i == 5 {
                assert_eq!(is_one_pair(&input[i]), true);
            } else {
                assert_eq!(is_one_pair(&input[i]), false);
            }
        }
    }
}
