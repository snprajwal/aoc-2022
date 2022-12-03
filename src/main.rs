mod days;

#[rustfmt::skip]
use days::{
    day_1, day_2, day_3, day_4, day_5,
    day_6, day_7, day_8, day_9, day_10,
    day_11, day_12, day_13, day_14, day_15,
    day_16, day_17, day_18, day_19, day_20,
    day_21, day_22, day_23, day_24, day_25,
};
use std::{env, fs};

fn main() {
    if env::args().count() < 2 {
        panic!("Please provide at least one day");
    }

    env::args()
        .skip(1)
        .map(|x| {
            x.parse::<u8>()
                .unwrap_or_else(|e| panic!("Invalid day: {e}"))
        })
        .for_each(|day| {
            let soln = solve(day);
            println!("=== Day {day} ===");
            println!("Part 1: {}", soln.0);
            println!("Part 2: {}", soln.1);
        })
}

fn solve(day: u8) -> (String, String) {
    let input = fs::read_to_string(format!("input/day_{day}.txt"))
        .unwrap_or_else(|e| panic!("Failed to read test cases for day {day}: {e}"));
    match day {
        1 => day_1::solve(input),
        2 => day_2::solve(input),
        3 => day_3::solve(input),
        4 => day_4::solve(input),
        5 => day_5::solve(input),
        6 => day_6::solve(input),
        7 => day_7::solve(input),
        8 => day_8::solve(input),
        9 => day_9::solve(input),
        10 => day_10::solve(input),
        11 => day_11::solve(input),
        12 => day_12::solve(input),
        13 => day_13::solve(input),
        14 => day_14::solve(input),
        15 => day_15::solve(input),
        16 => day_16::solve(input),
        17 => day_17::solve(input),
        18 => day_18::solve(input),
        19 => day_19::solve(input),
        20 => day_20::solve(input),
        21 => day_21::solve(input),
        22 => day_22::solve(input),
        23 => day_23::solve(input),
        24 => day_24::solve(input),
        25 => day_25::solve(input),
        _ => panic!("Invalid day {day}"),
    }
}
