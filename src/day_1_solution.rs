use std::fs;

pub fn day_one_solution() {
    println!("Running day 1 solution!");

    let contents = fs::read_to_string("./inputs/day-1-input");

    let mut sum = 0;

    for line in contents.unwrap().lines() {
        // println!("{line}");

        let chars = line.chars();
        let mut digits = Vec::new();
        for char in chars {
            // println!("{char}");

            if char.is_numeric() {
                digits.push(char);
            }
        }

        let first_char = digits.first().unwrap().to_string();
        let last_char = digits.last().unwrap().to_string();

        let mut line_num_string = String::new();
        line_num_string.push_str(&first_char);
        line_num_string.push_str(&last_char);
        // dbg!(&line_num_string);

        let line_num = line_num_string.parse::<u32>().unwrap();
        // dbg!(line_num);

        sum += line_num;
    }

    println!("Final sum:");
    println!("{sum}");
}