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
        const FILE01: &str = "/home/lui/RUST/advent_of_code/src/door01/input.txt";

        if let Ok(lines) = read_lines(FILE01) {
            for line in lines {
            }
        }

        println!("Door 01 part two result {}", sums[0] + sums[1] + sums[2]);
    }
}
