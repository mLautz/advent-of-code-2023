mod day_1_solution;

use std::env;
use crate::day_1_solution::day_one_solution;

fn main() {
    let args: Vec<String> = env::args().collect();

    // println!("Commands captured:");
    // dbg!(args);

    if args.len() != 2 {
        println!("Incorrect command line arguments. Please supply only a target day in the format of 'day_'");
        return;
    }

    let day = &args[1];
    // dbg!(day);

    match day.as_str(){
        "day1"=>day_one_solution(),
        _=>println!("No match for day provided.")
    }
}
