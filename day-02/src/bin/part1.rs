use std::fs::read_to_string;

#[derive(Debug)]
struct Game {
    id: u32,
    trys: Vec<Try>,
}

#[derive(Debug)]
struct Try {
    red: u32,
    green: u32,
    blue: u32,
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}

fn remove_first_char_if_whitespace(mut s: String) -> String {
    if s.chars().nth(0).unwrap() == ' ' {
        s = s[1..].to_string();
    }
    s
}

fn main() {
    let lines: Vec<String> = read_lines("input1.txt");
    let mut games: Vec<Game> = vec![];

    for line in lines {
        let mut game: Game = Game {
            id: 0,
            trys: vec![],
        };
        let game_str = line.split(":").collect::<Vec<&str>>();
        game.id = game_str[0].replace("Game ", "").parse().unwrap();
        let try_str = game_str[1].split(";").collect::<Vec<&str>>();
        for t in try_str {
            let mut tr: Try = Try {
                red: 0,
                green: 0,
                blue: 0,
            };
            let mod_t = &remove_first_char_if_whitespace(t.to_string());
            let color_str = mod_t.split(",").collect::<Vec<&str>>();
            for color in color_str {
                let c = &remove_first_char_if_whitespace(color.to_string());
                let cube = c.split(" ").collect::<Vec<&str>>();
                let cube_type = cube[1];
                let cube_amount = cube[0].parse().unwrap();

                if cube_type == "red" {
                    tr.red = cube_amount;
                }

                if cube_type == "green" {
                    tr.green = cube_amount;
                }
                if cube_type == "blue" {
                    tr.blue = cube_amount;
                }
            }
            game.trys.push(tr);
        }
        games.push(game);
    }

    let mut possible_games: Vec<u32> = vec![];

    for g in games {
        let mut game_possible = true;
        for t in g.trys {
            if t.red >= 12 || t.blue >= 14 || t.green >= 13 {
                game_possible = false;
            }
        }
        if game_possible {
            possible_games.push(g.id);
        }
    }

    let mut sum = 0;
    println!("possible games");
    for g in possible_games {
        print!("{} ", g);
        sum += g;
    }
    println!("\nsum {}", sum);
}
