use std::fs;

pub fn day_six_part_two_solution() {
    println!("Day 6 - Part 2");

    let contents = fs::read_to_string("./inputs/day-6-input").unwrap();

    let solutions = process_lines(contents);
    println!("Final output ===> {}", solutions);
}

fn process_lines(contents: String) -> u32 {
    let lines: Vec<&str> = contents.lines().collect();

    let times_line = lines[0];
    let distances_line = lines[1];

    let time = parse_line(&times_line);
    let distance = parse_line(&distances_line);
    dbg!(time);
    dbg!(distance);

    let mut options_count = 0;

    for hold_time in 0..time {
        if get_distance(time, hold_time) > distance {
            options_count += 1;
        }
    }

    return options_count;
}

fn parse_line (line: &str) -> u64 {
    return line.split(':').collect::<Vec<&str>>()[1].trim().replace(' ',"").parse::<u64>().unwrap();
}

fn get_distance(time: u64, hold: u64) -> u64 {
    return (time - hold) * hold;
}