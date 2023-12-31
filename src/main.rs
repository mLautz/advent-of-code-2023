mod day_1_part_1_solution;
mod day_1_part_2_solution;
mod day_2_part_1_solution;
mod day_2_part_2_solution;
mod day_3_part_1_solution;
mod day_4_part_1_solution;
mod day_4_part_2_solution;
mod day_6_part_1_solution;
mod day_6_part_2_solution;
mod day_7_part_1_solution;

use std::env;
use crate::day_1_part_1_solution::day_one_part_one_solution;
use crate::day_1_part_2_solution::day_one_part_two_solution;
use crate::day_2_part_1_solution::day_two_part_one_solution;
use crate::day_2_part_2_solution::day_two_part_two_solution;
use crate::day_3_part_1_solution::day_three_part_one_solution;
use crate::day_4_part_1_solution::day_four_part_one_solution;
use crate::day_4_part_2_solution::day_four_part_two_solution;
use crate::day_6_part_1_solution::day_six_part_one_solution;
use crate::day_6_part_2_solution::day_six_part_two_solution;
use crate::day_7_part_1_solution::day_seven_part_1_solution;

fn main() {
    let args: Vec<String> = env::args().collect();

    // println!("Commands captured:");
    // dbg!(args);

    if args.len() != 2 {
        println!("Incorrect command line arguments. Please supply only a target day in the format of 'day_part_'");
        return;
    }

    let day = &args[1];
    // dbg!(day);

    match day.as_str(){
        "day1part1"=> day_one_part_one_solution(),
        "day1part2"=> day_one_part_two_solution(),
        "day2part1"=> day_two_part_one_solution(),
        "day2part2"=> day_two_part_two_solution(),
        "day3part1"=> day_three_part_one_solution(),
        "day4part1"=> day_four_part_one_solution(),
        "day4part2"=> day_four_part_two_solution(),
        "day6part1"=> day_six_part_one_solution(),
        "day6part2"=> day_six_part_two_solution(),
        "day7part1"=> day_seven_part_1_solution(),
        _=>println!("No match for day provided.")
    }
}
