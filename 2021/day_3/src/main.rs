
use common::input_file_lines;

fn a() {
    let mut columns: Vec<String> = Vec::new();

    let mut gamma = 0;
    let mut epsilon = 0;

    let lines = input_file_lines();

    for line in lines {
        if let Ok(l) = line {
            for (i, c) in l.as_str().chars().enumerate() {
                if i >= columns.len() {
                    columns.push(String::from(""));
                }
                columns[i].push(c);
            }
        } else {
            continue;
        }
    }

    for column in columns {
        gamma <<= 1;
        epsilon <<= 1;

        let zeroes = column.chars().filter(|c| c == &'0').count();
        let ones = column.chars().filter(|c| c == &'1').count();
        match zeroes.cmp(&ones) {
            std::cmp::Ordering::Less => gamma += 1,
            _ => epsilon += 1,
        }
    }

    let result = gamma * epsilon;
    println!("Final Result A: {result} (gamma: {gamma}, epsilon: {epsilon})");
}

enum Digit {
    Zero,
    One,
    Both,
}

fn most_common_digit_at_index(source: &Vec<String>, selection: &Vec<usize>, index: usize) -> Digit {
    let mut counts: (i32, i32) = (0,0);
    for selection_index in selection {
        let digits: &String = &source[*selection_index];
        let digit = digits.chars().nth(index).expect("not a char");
        match digit {
            '0' => counts.0 += 1,
            '1' => counts.1 += 1,
            _ => panic!("unknown digit!"),
        }
    }
    match counts.0.cmp(&counts.1) {
        std::cmp::Ordering::Less => Digit::One,
        std::cmp::Ordering::Equal => Digit::Both,
        std::cmp::Ordering::Greater => Digit::Zero,
    }
}

fn b() {
    let lines = input_file_lines()
        .map(|l| l.expect("line failure"))
        .collect::<Vec<String>>();

    let mut oxygen_indices: Vec<usize> = Vec::from_iter(0..lines.len());
    let mut co2_indices: Vec<usize> = Vec::from_iter(0..lines.len());

    let mut look_index = 0;

    while (oxygen_indices.len() > 1 || co2_indices.len() > 1)
            && look_index < lines[0].len() {
        if oxygen_indices.len() > 1 {
            let oxygen_most_common = most_common_digit_at_index(&lines, &oxygen_indices, look_index);
            match oxygen_most_common {
                Digit::Zero => oxygen_indices.retain(
                    |s| lines[*s].chars().nth(look_index).unwrap() == '0'),
                _ =>  oxygen_indices.retain(
                    |s| lines[*s].chars().nth(look_index).unwrap() == '1'),
            }
        }

        if co2_indices.len() > 1 {
            let co2_most_common = most_common_digit_at_index(&lines, &co2_indices, look_index);
            match co2_most_common {
                Digit::Zero => co2_indices.retain(
                    |s| lines[*s].chars().nth(look_index).unwrap() == '1'),
                _ =>  co2_indices.retain(
                    |s| lines[*s].chars().nth(look_index).unwrap() == '0'),
            }
        }
        look_index += 1;
    }
    let oxygen_value_string = &lines[*oxygen_indices.get(0).unwrap()];
    let co2_value_string = &lines[*co2_indices.get(0).unwrap()];

    let oxygen_value = u32::from_str_radix(oxygen_value_string, 2).unwrap();
    let co2_value = u32::from_str_radix(co2_value_string, 2).unwrap();

    let result = oxygen_value * co2_value;

    println!("Final result B: {result} (oxygen: {oxygen_value}, co2: {co2_value})");
}

fn main() {
    a();
    b();
}
