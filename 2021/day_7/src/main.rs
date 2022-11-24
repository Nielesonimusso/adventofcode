use std::{cmp::Ordering, ops::RangeInclusive};

use common::input_file_lines;

fn a() {
    let mut crab_positions: Vec<i32> = input_file_lines()
        .next()
        .expect("no line")
        .expect("incorrect line")
        .split(",")
        .map(|cp| cp.parse().expect("Not correct position"))
        .collect();

    crab_positions.sort_unstable();

    let number_of_crabs = crab_positions.len();
    let center_of_crabs = crab_positions[number_of_crabs / 2];

    let fuel: i32 = crab_positions.iter()
        .map(|pos| (pos - center_of_crabs).abs())
        .sum();
    
    println!("Part A total fuel: {fuel}");
}

fn total_fuel_for_guess(positions: &Vec<i32>, guess: i32) -> i32 {
    positions.iter()
        .map(|pos| {
            let distance = (pos - guess).abs();
            (distance * (distance + 1)) / 2
        })
        .sum()
}

fn range_from_start_end(start: i32, end: i32) -> Vec<i32> {
    match start.cmp(&end) {
        Ordering::Greater => (start..=end).rev().collect(),
        _ => (start..=end).collect()
    }
}

fn min_fuel_for_range_b(positions: Vec<i32>, start: i32, end: i32) -> i32 {
    let mut min_fuel = i32::MAX;
    for guess in range_from_start_end(start, end) {
        let guess_fuel = total_fuel_for_guess(&positions, guess);
        // println!("guess {guess} fuel {guess_fuel} < {min_fuel} ?");
        match guess_fuel.cmp(&min_fuel) {
            Ordering::Less => min_fuel = guess_fuel,
            _ => break
        };
    }
    return min_fuel;
}

fn b() {
    let mut crab_positions: Vec<i32> = input_file_lines().next()
        .expect("no line")
        .expect("incorrect line")
        .split(",")
        .map(|cp| cp.parse().expect("Not correct position"))
        .collect();

    crab_positions.sort_unstable();
    
    // center = first guesses
    let number_of_crabs = crab_positions.len();
    let center_of_crabs = crab_positions[number_of_crabs / 2];

    let center_fuel: i32 = total_fuel_for_guess(&crab_positions, center_of_crabs);
    let right_fuel: i32 = total_fuel_for_guess(&crab_positions, center_of_crabs + 1);

    // println!("{center_fuel} <-> {right_fuel}");

    let min_fuel;
    match center_fuel.cmp(&right_fuel) {
        std::cmp::Ordering::Less => {
            // println!("Searching left");
            let min_of_crabs = *crab_positions.first().expect("no first?");
            min_fuel = min_fuel_for_range_b(crab_positions, center_of_crabs, min_of_crabs);
        },
        std::cmp::Ordering::Equal => {
            // println!("No search, middle will do");
            min_fuel = center_fuel;
        },
        std::cmp::Ordering::Greater => {
            // println!("Searching right");
            let max_of_crabs = *crab_positions.last().expect("no last?");
            min_fuel = min_fuel_for_range_b(crab_positions, center_of_crabs + 1, max_of_crabs);
        },
    }

    println!("Part B total fuel: {min_fuel}");
}

fn main() {
    a();
    b();
}
