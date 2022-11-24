use std::collections::VecDeque;
use std::cmp::Ordering;
use common::input_file_lines;

fn a() {
    let mut count = 0;
    let mut last_depth = 0;

    let lines = input_file_lines();

    for line in lines {
        if let Ok(l) = line {
            let depth: i32 = l.trim().parse().expect("no parse");
            match depth.cmp(&last_depth) {
                Ordering::Greater => count+=1,
                _ => (),
            }
            last_depth=depth;
        } else {
            continue;
        }
    }
    count -= 1;
    println!("{count} increases for part A");
}

fn b() {
    let mut count = 0;
    let mut windows: VecDeque<i32> = VecDeque::new();

    let lines = input_file_lines();
    
    for line in lines {
        if let Ok(l) = line {
            let depth: i32 = l.trim().parse().expect("no parse");
            windows.push_back(depth);
            if windows.len() == 4 {
                let window_a_sum: i32 = windows.range(..3).sum();
                let window_b_sum: i32 = windows.range(1..).sum();
                match window_b_sum.cmp(&window_a_sum) {
                    Ordering::Greater => count+=1,
                    _ => (),
                }
                windows.pop_front();
            }
        } else {
            continue;
        }
    }
    println!("{count} increases for part B");
}

fn main() {
    a();
    b();
}