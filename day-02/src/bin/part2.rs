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
    let lines: Vec<String> = read_lines("input2.txt");
    let mut games: Vec<Game> = vec![];
    let mut min_games: Vec<Game> = vec![];

    for line in lines {
        let mut game: Game = Game {
            id: 0,
            trys: vec![],
        };
        let game_str = line.split(":").collect::<Vec<&str>>();
        let game_id = game_str[0].replace("Game ", "").parse().unwrap();
        game.id = game_id;
        let try_str = game_str[1].split(";").collect::<Vec<&str>>();

        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;

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
                    if cube_amount > min_red {
                        min_red = cube_amount;
                    }
                }

                if cube_type == "green" {
                    tr.green = cube_amount;
                    if cube_amount > min_green {
                        min_green = cube_amount;
                    }
                }
                if cube_type == "blue" {
                    tr.blue = cube_amount;
                    if cube_amount > min_blue {
                        min_blue = cube_amount;
                    }
                }
            }
            game.trys.push(tr);
        }
        games.push(game);
        min_games.push(Game {
            id: game_id,
            trys: vec![Try {
                red: min_red,
                green: min_green,
                blue: min_blue,
            }],
        })
    }

    let mut sum = 0;
    println!("possible games");
    for g in min_games {
        println!("Game: {}, Trys: {}", g.id, g.trys.len());
        let p = g.trys[0].red * g.trys[0].blue * g.trys[0].green;
        sum += p;
    }
    println!("\nsum {}", sum);
}
