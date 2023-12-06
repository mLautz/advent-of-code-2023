use std::fs;
use regex::Regex;

pub fn day_four_part_one_solution() {
    println!("Day 4 - Part 1");

    let contents = fs::read_to_string("./inputs/day-4-input");

    let mut card_sum = 0;

    for line in contents.unwrap().lines() {
        let line_sans_card_num: Vec<&str> = line.split(&[':','|'][..]).collect();

        let winning_num_string = line_sans_card_num.get(1).unwrap();
        let eligible_num_string = line_sans_card_num.get(2).unwrap();

        let winning_nums = extract_digits(winning_num_string);
        let eligible_nums = extract_digits(eligible_num_string);

        let mut card_power:i32 = -1;
        winning_nums.iter().for_each(|num| {
            if eligible_nums.contains(num) {
                card_power += 1;
            }
        });

        card_sum += if card_power >= 0{ 2_i32.pow(card_power as u32) } else { 0 };
    }

    println!("\n\nFinal Card Sum ====> {}", card_sum);
}

fn extract_digits(num_string: &str) -> Vec<u32> {
    let digit_regex = Regex::new(r"\d+").unwrap();

    let mut nums = Vec::new();

    for (num_match, []) in digit_regex.captures_iter(num_string).map(|cap| cap.extract()) {
        let parsed_num = num_match.parse::<u32>().unwrap();
        nums.push(parsed_num);
    }

    return nums;
}