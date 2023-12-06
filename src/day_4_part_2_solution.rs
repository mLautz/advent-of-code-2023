use std::fs;
use regex::Regex;

pub fn day_four_part_two_solution() {
    println!("Day 4 - Part 1");

    let contents = fs::read_to_string("./inputs/day-4-input").unwrap();
    let line_mapping: Vec<u32> = process_cards(&contents);

    let card_count: Vec<u32> = (0..(contents.lines().count() as u32)).collect();
    let mut total_card_count: u32 = 0;

    card_count.into_iter().for_each(|card| {
        total_card_count += evaluate_cards(&line_mapping, card);
    });

    println!("\n\nFinal Card Sum ====> {}", total_card_count);
}

fn process_cards(content: &String) -> Vec<u32> {
    let mut line_mapping: Vec<u32> = Vec::new();

    for line in content.lines() {
        let line_sans_card_num: Vec<&str> = line.split(&[':','|'][..]).collect();

        let winning_num_string = line_sans_card_num.get(1).unwrap();
        let eligible_num_string = line_sans_card_num.get(2).unwrap();

        let winning_nums = extract_digits(winning_num_string);
        let eligible_nums = extract_digits(eligible_num_string);

        let mut match_count = 0;
        winning_nums.iter().for_each(|num| {
            if eligible_nums.contains(num) {
                match_count += 1;
            }
        });

        line_mapping.push(match_count);
    }

    return line_mapping;
}

fn evaluate_cards(line_mapping: &Vec<u32>, current_card: u32) ->  u32{
    // println!("\n\nEvaluating new line: {}", current_card);
    let winning_count_lookup: Option<&u32> = line_mapping.get(current_card as usize);
    let winning_count: u32 = winning_count_lookup.unwrap_or(&0).clone();

    let mut card_count = 1;

    let mut subsequent_card_index = 0;
    (0..winning_count).collect::<Vec<u32>>().iter().for_each(|_| {
        subsequent_card_index += 1;
        card_count += evaluate_cards(line_mapping, current_card + subsequent_card_index);
    });

    // println!("Returning card count from line {} --> {}", current_card, card_count);
    return card_count;
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