
use common::input_file_lines;

fn read_first_generation(s: String) -> [i64; 9] {
    let mut result = [0; 9];
    for f in s.split(",")
            .map(|fs| fs.parse::<usize>().expect("not number")) {
        result[f] += 1;
    }
    result
}

fn day(input: &mut [i64; 9]) {
    let new_gen = input[0];
    for i in 0..8 {
        input[i] = input[i+1];
    }
    input[6] += new_gen;
    input[8] = new_gen;
}

fn a() {
    let mut lines = input_file_lines();

    let mut population = read_first_generation(lines.next()
        .expect("no population")
        .expect("invalid population"));

    for _ in 0..80 {
        day(&mut population);
        // println!("{population:?}");
    }

    let population_sum: i64 = population.iter().sum();
    println!("Part A population: {population_sum} ({population:?})");
}

fn b() {
    let mut lines = input_file_lines();

    let mut population = read_first_generation(lines.next()
        .expect("no population")
        .expect("invalid population"));

    for _ in 0..256 {
        day(&mut population);
        // println!("{population:?}");
    }

    let population_sum: i64 = population.iter().sum();
    println!("Part B population: {population_sum} ({population:?})");
}

fn main() {
    a();
    b();
}
