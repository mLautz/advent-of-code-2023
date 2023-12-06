use std::collections::HashSet;
use std::fs;
use regex::{CaptureMatches, Regex};

pub fn day_three_part_one_solution() {
    println!("Day 3 - Part 1");

    let contents = fs::read_to_string("./inputs/day-3-input").unwrap();
    let search_regex = Regex::new(r"([*]|\d+|[^0-9A-Za-z.])").unwrap();

    let mut line_count: u32 = 1;

    let mut data_map: Vec<(u32, Vec<(u32, Vec<u32>)>, Vec<u32>)> = Vec::new();

    contents.lines().for_each(|line| {
        let search_results = search_regex.captures_iter(line);

        let processed_results = process_capture(line_count, line, search_results);

        data_map.push(processed_results);
        line_count += 1;
    });

    // dbg!(&data_map);

    let mut rolling_sum = 0;
    let mut prior_syms: Option<Vec<u32>> = None;

    data_map.iter().for_each(|row| {
        // GET CURRENT NUMBERS
        let row_number_matches = row.1.to_vec();

        // CHECK FOR PRIOR, CURRENT, FUTURE SYMBOLS
        let mut symbol_indices: Vec<u32> = Vec::new();

        if prior_syms.is_some() {
            // dbg!(&prior_syms);
            symbol_indices.append(&mut prior_syms.clone().unwrap());
        };

        let current_syms = row.2.to_vec();
        // dbg!(&current_syms);
        symbol_indices.append(&mut current_syms.to_vec());

        let next_row = data_map.get((row.0) as usize);
        if next_row.is_some() {
            // dbg!(&next_row);
            symbol_indices.append(&mut next_row.clone().unwrap().2.to_vec());
        }

        // dbg!(&symbol_indices);

        // EXPAND INDICES MATCH AREA
        let mut sym_index_set: HashSet<u32> = HashSet::new();
        symbol_indices.into_iter().for_each(|sym_index| {
            if sym_index > 0 {
                sym_index_set.insert(sym_index - 1);
            }
            sym_index_set.insert(sym_index);
            sym_index_set.insert(sym_index + 1);
        });
        // dbg!(&sym_index_set);

        // CHECK NUM INDICES AGAINST SYM INDICES

        row_number_matches.iter().for_each(|num_match| {
            println!("\nChecking for {} on {}", &num_match.0, row.0);

            let mut match_found = false;
            num_match.1.iter().for_each(|num_index| {
                if sym_index_set.contains(num_index) {
                    match_found = true;
                }
            });

            if match_found {
                println!("Adding to sum => {}", &num_match.0);
                rolling_sum += &num_match.0;
            }
        });

        prior_syms = Some(current_syms.to_vec());
        // dbg!(&prior_syms);
    });

    println!("Final Sum ====> {}", rolling_sum);
}

fn process_capture(line_count: u32, line: &str, search_results: CaptureMatches) -> (u32, Vec<(u32, Vec<u32>)>, Vec<u32>){
    let mut number_matches = Vec::new();
    let mut symbol_matches = Vec::new();

    search_results.for_each(|cap| {
        let (_, [captured_value]) = cap.extract();

        let match_index_set: Vec<(usize, &str)> = line.match_indices(&captured_value).collect();

        if captured_value.len() == 1 && !captured_value.as_bytes()[0].is_ascii_digit() {
            // println!("Symbol => {} @ {},{}", &captured_value, &line_count, &index_of_match);
            match_index_set.iter().for_each(|match_index| {
                symbol_matches.push(match_index.0 as u32);
            });
            // symbol_matches.push(index_of_match);
        } else {
            // println!("Number => {}", &captured_value);
            let captured_num = captured_value.parse::<u32>().unwrap();

            match_index_set.iter().for_each(|match_index| {
                let min_index = match_index.0 as u32;
                let max_index: u32 = min_index + captured_value.chars().count() as u32;

                let matching_indices: Vec<u32> = (min_index..max_index).collect();

                number_matches.push((captured_num, matching_indices))
            });
        }
    });

    return (line_count, number_matches, symbol_matches);
}