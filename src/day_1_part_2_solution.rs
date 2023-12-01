use std::collections::BTreeMap;
use std::fs;

pub fn day_one_part_two_solution() {
    println!("Running day 1, part 2 solution!");

    let contents = fs::read_to_string("./inputs/day-1-input");

    let mut sum = 0;

    for line in contents.unwrap().lines() {
        // println!("{line}");

        sum += find_num_sum(line);
    }

    println!("Final sum:");
    println!("{sum}");
}

fn find_num_sum(search_string: &str) -> u32 {

    let search_contents = ["1", "2", "3", "4", "5", "6", "7", "8", "9", "0", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "zero"];
    let mut result_map = BTreeMap::new();

    for target in search_contents {
        let matches = search_string.match_indices(target);

        for (index, value) in matches {
            result_map.insert(index, value);
        }
    }

    let last_result = result_map.iter().last().unwrap().1;
    let first_result = result_map.iter().next().unwrap().1;

    let first_num_string = convert_string_to_num(first_result);
    let last_num_string = convert_string_to_num(last_result);

    let mut line_num_string = String::new();
    line_num_string.push_str(&first_num_string);
    line_num_string.push_str(&last_num_string);

    let line_num = line_num_string.parse::<u32>().unwrap();
    // println!("=> {line_num}");

    return line_num;
}

fn convert_string_to_num(input: &str) -> &str {
    if input.len() == 1 {
        return input;
    }

    let converted_val = match input {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        "zero" => "0",
        _ => "-1"
    };

    return converted_val;
}