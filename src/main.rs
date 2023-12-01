mod day_1_part_1_solution;
mod day_1_part_2_solution;

use std::env;
use crate::day_1_part_1_solution::day_one_part_one_solution;
use crate::day_1_part_2_solution::day_one_part_two_solution;

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
        _=>println!("No match for day provided.")
    }
}
