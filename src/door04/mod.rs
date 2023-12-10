pub mod solution {
    use std::collections::HashMap;
    use std::env;
    use std::path::{Path, PathBuf};
    use crate::read_lines::read_lines::read_lines;

    pub fn part_one() {
        let path = env::current_dir().unwrap();
        let file04 = path.join("./src/door04/input.txt");
        let tutorial_input = path.join("./src/door04/tutorial_input.txt");


        assert_eq!(13, process_file(tutorial_input));
        process_file(file04);
    }

    pub fn part_two() {
        let path = env::current_dir().unwrap();
        let file04 = path.join("./src/door04/input.txt");

        process_file_part_2(file04);
    }

    fn process_file_part_2(file: PathBuf) -> i32 {
        let mut sum: i32 = 0;
        let mut card_copies: Vec<i32> = generate_card_count_vec(file.to_owned());

        if let Ok(lines) = read_lines(file) {
            let mut current_index = 0;
            for line in lines {
                let mut current_sum = 0;
                // add multiplication for each game
                let line_unwrap = line.unwrap();
                let mut all_numbers = line_unwrap.split(":").last().unwrap().split("|");
                let winning_numbers: Vec<&str> = all_numbers.next().unwrap().split_whitespace().collect();
                let scratched_numbers: Vec<&str> = all_numbers.last().unwrap().split_whitespace().collect();

                let mut wins_count = 0; // one for current card
                for scratched_number in &scratched_numbers {
                    for winning_number in &winning_numbers {
                        if winning_number == scratched_number {
                            wins_count = wins_count + 1;
                        }
                    }
                }
                // process wins for next games
                for win in 1..wins_count + 1 {
                    if win > card_copies.len() {
                        continue;
                    }
                    card_copies[current_index + win] = card_copies[current_index + win] + card_copies[current_index];
                }
                current_sum = current_sum + card_copies[current_index];
                sum = sum + current_sum;
                println!("Game {} has {} matching numbers and had multiplication {}", current_index, wins_count, card_copies[current_index]);
                current_index = current_index + 1;
            }
        }

        println!("Door 04 part two result {}", sum);
        return sum;
    }

    fn generate_card_count_vec(file: PathBuf) -> Vec<i32> {
        let mut vec: Vec<i32> = vec![0; 0];

        if let Ok(lines) = read_lines(file) {
            vec = vec![1; lines.count()];
        }
        return vec;
    }

    fn process_file(file: PathBuf) -> i32 {
        let mut current_sum: i32 = 0;

        if let Ok(lines) = read_lines(file) {
            for line in lines {
                let line_unwrap = line.unwrap();
                let mut all_numbers = line_unwrap.split(":").last().unwrap().split("|");
                let winning_numbers: Vec<&str> = all_numbers.next().unwrap().split_whitespace().collect();
                let scratched_numbers: Vec<&str> = all_numbers.last().unwrap().split_whitespace().collect();

                let mut current_win = 0;
                for scratched_number in &scratched_numbers {
                    for winning_number in &winning_numbers {
                        if winning_number == scratched_number {
                            if current_win == 0 {
                                current_win = 1;
                            } else {
                                current_win = current_win * 2;
                            }
                        }
                    }
                }
                println!("Win of game {}", current_win);
                current_sum = current_sum + current_win;
            }
        }

        println!("Door 04 part one result {}", current_sum);
        return current_sum;
    }
}