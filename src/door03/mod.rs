pub mod solution {
    use std::env;
    use std::path::PathBuf;
    use crate::door03::is_connected_to_gear;
    use crate::read_lines::read_lines::read_lines;

    pub fn part_one() {
        let path = env::current_dir().unwrap();
        let tutorial_input = path.join("./src/door03/tutorial_input.txt");
        let test0 = path.join("./src/door03/test_0.txt");
        let test_horizontal = path.join("./src/door03/test_horizontal.txt");
        let test_vertical = path.join("./src/door03/test_vertical.txt");
        let test_diagonal = path.join("./src/door03/test_diagonal.txt");
        let file03 = path.join("./src/door03/input.txt");
        let other_test = path.join("./src/door03/other_test.txt");

        assert_eq!(0, process_file(test0));
        assert_eq!(88, process_file(test_horizontal));
        assert_eq!(110, process_file(test_vertical));
        assert_eq!(132, process_file(test_diagonal));
        assert_eq!(4361, process_file(tutorial_input));
        assert_eq!(7, process_file(other_test));
        assert_eq!(527446, process_file(file03));
    }

    pub fn part_two() {
        let path = env::current_dir().unwrap();
        let file03 = path.join("./src/door03/input.txt");

        process_file_part_2(file03);
    }

    fn process_file(file: PathBuf) -> i32 {
        let mut current_sum: i32 = 0;

        let mut grid = generate_grid(file);

        for (i, grid_line) in grid.iter().enumerate() {
            let mut number = String::new();
            let mut start_index = 0;

            for (ii, grid_char) in grid_line.iter().enumerate() {
                if grid_char.is_digit(10) {
                    number.push(*grid_char);
                    if number.len() == 1 {
                        start_index = ii;
                    }
                }
                if (!grid_char.is_digit(10) && number.len() > 0) ||
                    (number.len() > 0 && ii == grid_line.len() - 1) {
                    println!("Current number {}", number);
                    let len = number.len();
                    let mut x = start_index;
                    let y = i;
                    // Check chars around number
                    let number_counts = check_if_number_should_count((*grid).to_owned(), x, y, len);

                    if number_counts {
                        current_sum = current_sum + number.parse::<i32>().unwrap();
                    }
                    println!("Current sum {}", current_sum);
                    // Reset
                    number = String::new();
                }
            }
        }

        println!("Door 03 part one result {}", current_sum);
        return current_sum;
    }

    fn process_file_part_2(file: PathBuf) -> i32 {
        let mut current_sum: i32 = 0;

        let mut grid = generate_grid(file);

        for (y, grid_line) in grid.iter().enumerate() {
            for (x, grid_char) in grid_line.iter().enumerate() {
                if grid_char == &'*' {
                    let product_of_two_numbers = check_numbers_around_gear(&grid, x as i32, y as i32);
                    current_sum = current_sum + product_of_two_numbers;
                }
            }
        }

        println!("Door 03 part two result {}", current_sum);
        return current_sum;
    }

    pub fn generate_grid(file: PathBuf) -> Vec<Vec<char>> {
        let mut grid = Vec::new();
        if let Ok(lines) = read_lines(file) {
            for line in lines {
                let grid_line = line.unwrap().chars().collect();
                grid.push(grid_line);
            }
        }
        return grid;
    }

    pub fn check_if_number_should_count(grid: Vec<Vec<char>>, x: usize, y: usize, len: usize) -> bool {
        println!("Check x {} y {}", x, y);
        let iterations = if x == 0 { len + 1 } else { len + 2 };
        // check row above if y > 0
        if y > 0 {
            for i in 0..iterations {
                let current_y = y - 1;
                let mut current_x = x + i;
                // if x is not at the beginning of the row start one to the left
                if x != 0 {
                    current_x = current_x - 1;
                }
                // if current position is the end of the line
                if current_x == grid[y].len() {
                    break;
                }
                println!("check row above if y > 0 y {} x {}", current_y, current_x);
                let current_char = grid[current_y][current_x];
                if current_char != '.' && !current_char.is_digit(10) {
                    println!("Found char {}", current_char);
                    return true;
                }
            }
        }
        // check row below if y < grid length
        if y < grid.len() - 1 {
            for i in 0..iterations {
                let current_y = y + 1;
                let mut current_x = x + i;
                // if x is not at the beginning of the row start one to the left
                if x != 0 {
                    current_x = current_x - 1;
                }
                // if current position is the end of the line
                if current_x == grid[y].len() {
                    break;
                }
                println!("check row below if y < grid length y {} x {}", current_y, current_x);
                let current_char = grid[current_y][current_x];
                if current_char != '.' && !current_char.is_digit(10) {
                    println!("Found char {}", current_char);
                    return true;
                }
            }
        }
        // check before in same row
        println!("check before in same row");
        if x > 0 {
            let current_char = grid[y][x - 1];
            if current_char != '.' && !current_char.is_digit(10) {
                println!("Found char {}", current_char);
                return true;
            }
        }
        // check after in same row
        println!("check after in same row");
        if x + len < grid[y].len() {
            let current_char = grid[y][x + len];
            if current_char != '.' && !current_char.is_digit(10) {
                println!("Found char {}", current_char);
                return true;
            }
        }
        println!("NOTHING FOUND!");
        return false;
    }

    pub fn check_numbers_around_gear(grid: &Vec<Vec<char>>, x_input: i32, y_input: i32) -> i32 {
        // println!("Check x {} y {}", x_input, y_input);
        let start_x = x_input - 3;
        let end_x = x_input + 4;
        let start_y = y_input - 1;
        let end_y = y_input + 2;

        let mut number_one = String::new();
        let mut number_two = String::new();

        let mut number_one_finished = false;
        let mut number_two_finished = false;
        let mut number_one_connected_to_gear = false;
        let mut number_two_connected_to_gear = false;

        for y in start_y..end_y {
            if y > grid.len() as i32 - 1 || y < 0 {
                continue;
            }
            let mut x_counter = 0;
            for x in start_x..end_x {
                if x < 0 {
                    continue;
                }
                // line is finished so handle number_one and number_two
                if x > grid[usize::try_from(y).unwrap()].len() as i32 - 1 {
                    if !number_one.is_empty() && !number_one_finished {
                        if number_one_connected_to_gear {
                            number_one_finished = true;
                        }
                    } else if !number_two.is_empty() && !number_two_finished {
                        if number_two_connected_to_gear {
                            number_two_finished = true;
                        }
                    }
                    continue;
                }


                let grid_char = grid[usize::try_from(y).unwrap()][usize::try_from(x).unwrap()];

                if grid_char.is_digit(10) {
                    if number_one_finished && number_two_finished { // more than one number around gear is invalid
                        return number_one.parse::<i32>().unwrap() * number_two.parse::<i32>().unwrap();
                    }
                    if !number_one_finished {
                        number_one.push(grid_char);
                        if !number_one_connected_to_gear {
                            number_one_connected_to_gear = is_connected_to_gear(x_input, y_input, x, y);
                        }
                        if x_counter > 5 {
                            number_one_finished = true;
                        }
                    } else {
                        number_two.push(grid_char);
                        if !number_two_connected_to_gear {
                            number_two_connected_to_gear = is_connected_to_gear(x_input, y_input, x, y);
                        }
                        if x_counter > 5 {
                            number_two_finished = true;
                        }
                    }
                } else { // any other character
                    if !number_one.is_empty() && !number_one_finished {
                        if number_one_connected_to_gear {
                            number_one_finished = true;
                        } else { // reset
                            number_one = String::new();
                        }
                    } else if !number_two.is_empty() && !number_two_finished {
                        if number_two_connected_to_gear {
                            number_two_finished = true;
                        } else { // reset
                            number_two = String::new();
                        }
                    }
                    if x_counter > 3 { // there is no number anymore
                        break;
                    }
                }
                x_counter = x_counter + 1;
            }
        }

        if !number_one.is_empty() && !number_two.is_empty() {
            println!("Found numbers {} and {}", number_one, number_two);
            return number_one.parse::<i32>().unwrap() * number_two.parse::<i32>().unwrap();
        }
        return 0;
    }
}

pub fn is_connected_to_gear(gear_x: i32, gear_y: i32, x: i32, y: i32) -> bool {
    return !(gear_x.abs_diff(x) > 1) && !(gear_y.abs_diff(y) > 1);
}
