pub mod solution {
    use crate::read_lines::read_lines::read_lines;

    pub fn part_one() {
        const FILE01: &str = "/home/lui/RUST/advent_of_code_2023/src/door01/input.txt";
        let mut current_sum: i32 = 0;

        if let Ok(lines) = read_lines(FILE01) {
            for line in lines {
                let mut all_numbers: String = line.unwrap().chars().filter(|c| c.is_digit(10)).collect();
                if all_numbers.len() > 1 {
                    let first_number = all_numbers.chars().next().unwrap();
                    let last_number = all_numbers.chars().last().unwrap();
                    all_numbers = format!("{}{}", first_number, last_number);
                } else if all_numbers.len() == 1 {
                    let first_number = all_numbers.chars().next().unwrap();
                    all_numbers = format!("{}{}", first_number, first_number);
                }
                if let Ok(value) = all_numbers.parse::<i32>() {
                    current_sum = current_sum + value;
                }
            }
        }
        println!("Door 01 part one result {}", current_sum);
    }

    pub fn part_two() {
        const FILE01: &str = "/home/lui/RUST/advent_of_code_2023/src/door01/input.txt";
        let string_numbers: [&str; 9] = [
            "one",
            "two",
            "three",
            "four",
            "five",
            "six",
            "seven",
            "eight",
            "nine"
        ];
        let mut current_sum: i32 = 0;

        if let Ok(lines) = read_lines(FILE01) {
            for line in lines {
                let mut line_unwrap = line.unwrap();
                let mut number_array:Vec<char> = vec!['0'; line_unwrap.len()];

                for (i, char) in line_unwrap.chars().enumerate() {
                    if char.is_digit(10) {
                        number_array[i] = char;
                    }
                };

                for (string_number_index,string_number) in string_numbers.iter().enumerate() {
                    let mut word_positions: Vec<_> = line_unwrap.match_indices(string_number).map(|(i, _)|i).collect();
                    for word_position in word_positions {
                        let value = string_number_index + 1;
                        number_array[word_position] = char::from_digit(value as u32, 10).unwrap();
                    }
                }
                number_array.retain(|&x| x != '0');
                let mut all_numbers = "".to_string();

                if number_array.len() > 1 {
                    let first_number = number_array.first().unwrap();
                    let last_number = number_array.last().unwrap();
                    all_numbers = format!("{}{}", first_number, last_number);
                } else if number_array.len() == 1 {
                    let first_number = number_array.first().unwrap();
                    all_numbers = format!("{}{}", first_number, first_number);
                }
                println!("Input :{} \n Output: {}", line_unwrap, all_numbers);
                if let Ok(value) = all_numbers.parse::<i32>() {
                    current_sum = current_sum + value;
                }
            }
        }

        println!("Door 01 part two result {}", current_sum);
    }
}
