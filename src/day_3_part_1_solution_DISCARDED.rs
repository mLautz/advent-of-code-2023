use std::fs;
use regex::Regex;

pub fn day_three_part_one_solution() {
    println!("Day 3 - Part 1");

    let contents = fs::read_to_string("./inputs/day-3-input-example").unwrap();
    let search_regex = Regex::new(r"(\d+)|([^0-9A-Za-z\.])").unwrap();

    let mut line_count: u32 = 1;

    let mut data_map: Vec<(u32, Vec<(u32, Vec<u32>)>, Vec<u32>)> = Vec::new();

    contents.lines().for_each(|line| {
        let search_results = search_regex.captures_iter(line);

        let mut number_matches = Vec::new();
        let mut symbol_matches = Vec::new();

        search_results.for_each(|cap| {
            let (_, [captured_value]) = cap.extract();

            let index_of_match = line.find(&captured_value).unwrap() as u32;

            if captured_value.len() == 1 && !captured_value.as_bytes()[0].is_ascii_digit() {
                // println!("Symbol => {}", &captured_value);
                symbol_matches.push(index_of_match);
            } else {
                // println!("Number => {}", &captured_value);
                let captured_num = captured_value.parse::<u32>().unwrap();
                let max_index: u32 = index_of_match + captured_value.chars().count() as u32;

                let matching_indices: Vec<u32> = (index_of_match..max_index).collect();
                number_matches.push((captured_num, matching_indices))
            }
        });


        data_map.push((line_count, number_matches, symbol_matches));
        line_count += 1;
    });

    // dbg!(&data_map);

    let mut rolling_sum = 0;

    data_map.iter().for_each(|row| {
        let current_symbols = row.2.to_vec();

        let prev_row_index = row.0 as i32 - 2;
        if prev_row_index < 0 { return; }

        let prev_row_lookup = data_map.get(prev_row_index as usize);

        if let Some(prev_row) = prev_row_lookup {
            let prev_numbers = prev_row.1.to_vec();

            let symbol_index_set: Vec<u32>  = current_symbols.into_iter().map(|sym_index| {
                [sym_index - 1, sym_index, sym_index + 1]
            }).flatten().collect();

            prev_numbers.iter().for_each(|num_result| {
                let num_found = num_result.0;
                let num_indices: Vec<u32> = num_result.1.to_vec();
                

                let has_match = num_indices.into_iter().find(|num_index| {
                    symbol_index_set.contains(num_index)
                });

                if has_match.is_some() {
                    rolling_sum += num_found;
                }
            });
        }
    });

    println!("Final Sum ====> {}", rolling_sum);
}