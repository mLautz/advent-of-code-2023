use std::fs;

pub fn day_six_part_one_solution() {
    println!("Day 6 - Part 1");

    let contents = fs::read_to_string("./inputs/day-6-input").unwrap();

    let solutions_product = process_lines(contents);
    println!("Final product ===> {}", solutions_product);
}

fn process_lines(contents: String) -> u32 {
    let lines: Vec<&str> = contents.lines().collect();

    let times_line = lines[0];
    let distances_line = lines[1];

    let times = parse_line(&times_line);
    let distances = parse_line(&distances_line);

    let zip_pairs = times.iter().zip(distances.iter());

    let time_dist_pairs: Vec<(u32, u32)> = zip_pairs.map(|pair| {
        let time_parse = pair.0.parse::<u32>().unwrap();
        let distance_parse = pair.1.parse::<u32>().unwrap();
        (time_parse, distance_parse)
    }).collect();

    let mut options_product = 1;

    for pair in time_dist_pairs {
        let time = pair.0;
        let distance = pair.1;

        let mut options_count = 0;

        for hold_time in 0..time {
            if get_distance(time, hold_time) > distance {
                options_count += 1;
            }
        }

        options_product *= options_count;
    }

    return if options_product != 1 { options_product } else { 0 };
}

fn parse_line (line: &str) -> Vec<&str> {
    return line.split(':').collect::<Vec<&str>>()[1].trim().split_whitespace().collect::<Vec<&str>>();
}

fn get_distance(time: u32, hold: u32) -> u32 {
    return (time - hold) * hold;
}