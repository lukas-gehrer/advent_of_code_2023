pub mod solution {
    use crate::read_lines::read_lines::read_lines;
    use std::collections::HashMap;
    use std::env;

    pub fn part_one() {
        let path = env::current_dir().unwrap();
        let file02 = path.join("./src/door02/input.txt");

        let mut current_sum: i32 = 0;
        let loaded_colors = HashMap::from([
            ("red", 12),
            ("green", 13),
            ("blue", 14)
        ]);

        if let Ok(lines) = read_lines(file02) {
            for line in lines {
                let line_unwrap = line.unwrap();
                let game = line_unwrap.split(":").next().unwrap();
                let game_number: String = game.chars().filter(|c| c.is_digit(10)).collect();
                let games = line_unwrap.split(":").last().unwrap();
                let single_games = games.split(";");

                let mut game_allowed = true;
                for single_game in single_games {
                    let colors = single_game.split(",");
                    for color in colors {
                        let current_color = color.split(" ").last().unwrap();
                        let color_count: String = color.chars().filter(|c| c.is_digit(10)).collect();
                        if color_count.parse::<i32>().unwrap() > loaded_colors[current_color]{
                            game_allowed = false;
                            break;
                        }
                    }
                    if !game_allowed {
                        break;
                    }
                }
                if game_allowed {
                    current_sum = current_sum + game_number.parse::<i32>().unwrap();
                }
            }
        }
        println!("Door 02 part one result {}", current_sum);
    }

    pub fn part_two() {
        let path = env::current_dir().unwrap();
        let file02 = path.join("./src/door02/input.txt");

        let mut sum: i32 = 0;

        if let Ok(lines) = read_lines(file02.to_str().unwrap()) {
            for line in lines {
                let mut fewest_colors = HashMap::from([
                    ("red", 0),
                    ("green", 0),
                    ("blue", 0)
                ]);

                let line_unwrap = line.unwrap();
                let games = line_unwrap.split(":").last().unwrap();
                let single_games = games.split(";");

                for single_game in single_games {
                    let colors = single_game.split(",");
                    for color in colors {
                        let current_color = color.split(" ").last().unwrap();
                        let color_count: String = color.chars().filter(|c| c.is_digit(10)).collect();
                        if fewest_colors[current_color] < color_count.parse::<i32>().unwrap() {
                            *fewest_colors.get_mut(current_color).unwrap() = color_count.parse::<i32>().unwrap();
                        }
                    }
                }
                let current_sum = fewest_colors["red"] * fewest_colors["green"] * fewest_colors["blue"];
                sum = sum + current_sum;
            }
        }

        println!("Door 02 part two result {}", sum);
    }
}
