use regex::Regex;
use std::fs;

pub fn day_two_part_two_solution() {
    println!("Day 2 - Part 1");

    let contents = fs::read_to_string("./inputs/day-2-input");
    let target_max = (13, 12, 14); //(green, red, blue)
    let mut game_sum = 0;

    for line in contents.unwrap().lines() {
        let line_split: Vec<&str> = line.split(":").collect();
        let game_str = line_split[0];
        let game_num = Regex::new(r"\d+").unwrap().find(game_str).unwrap().as_str().parse::<u32>().unwrap();

        println!("Game number: {game_num}");

        let pull_sets: Vec<&str> = line_split[1].split(";").collect();

        let green_regex = Regex::new(r"(?<number>\d+) (?<green>green)").unwrap();
        let red_regex = Regex::new(r"(?<number>\d+) (?<red>red)").unwrap();
        let blue_regex = Regex::new(r"(?<number>\d+) (?<blue>blue)").unwrap();

        // green, red, blue
        let mut color_max = (0, 0, 0);

        for set in pull_sets {
            let set_string = set.to_string();
            let colors: Vec<&str> = set_string.split(",").collect();

            // (green, red, blue)
            // let mut color_values = (0, 0, 0);

            for color in colors {
                green_regex.captures(color).map(|cap|{
                    let green_amount = cap.name("number").unwrap().as_str().parse::<u32>().unwrap();
                    // color_values = (green_amount, color_values.1, color_values.2);

                    if green_amount > color_max.0 { color_max = (green_amount, color_max.1, color_max.2) }
                });

                red_regex.captures(color).map(|cap|{
                    let red_amount = cap.name("number").unwrap().as_str().parse::<u32>().unwrap();
                    // color_values = (color_values.0, red_amount, color_values.2);

                    if red_amount > color_max.1 { color_max = (color_max.0, red_amount, color_max.2) }
                });

                blue_regex.captures(color).map(|cap|{
                    let blue_amount = cap.name("number").unwrap().as_str().parse::<u32>().unwrap();
                    // color_values = (color_values.0, color_values.1, blue_amount);

                    if blue_amount > color_max.2 { color_max = (color_max.0, color_max.1, blue_amount) }
                });
            }

            // println!("=> Green: {}, red: {}, blue: {}", color_values.0, color_values.1, color_values.2);
        }

        println!("Game color maximums - Green: {}, red: {}, blue: {}", color_max.0, color_max.1, color_max.2);

        let power = color_max.0 * color_max.1 * color_max.2;
        game_sum += power;
    }

    println!("Final game number total: {game_sum}");
}