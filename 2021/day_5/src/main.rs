use std::{collections::{HashMap}, cmp::{max, min}, ops::Range};

use common::input_file_lines;

#[derive(Debug)]
struct VentLine {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

impl VentLine {
    fn read_line(line: String) -> Self {
        
        let parts: Vec<&str> = line.split(" ").into_iter().collect();
        let start = parts[0].split_once(",")
            .expect("not two start coordinates");
        let end = parts[2].split_once(",")
                .expect("not two end coordinates");

        VentLine {
            x1: start.0.parse().expect("no good start x"), 
            y1: start.1.parse().expect("no good start y"), 
            x2: end.0.parse().expect("no good end x"), 
            y2: end.1.parse().expect("no good end y")
        }
    }
}

fn int_range_from_numbers(a: &i32, b: &i32) -> Vec::<i32> {
    match a.cmp(b) {
        std::cmp::Ordering::Greater => (*b..=*a).into_iter().rev().collect(),
        _ => (*a..=*b).collect()
    }
}

fn a() {
    let mut map: HashMap<(i32, i32), i32>= HashMap::new();

    let lines = input_file_lines();

    for line in lines {
        let vent_line = VentLine::read_line(line.expect("no line"));

        if vent_line.x1 == vent_line.x2 { // vertical
            // println!("vertical vent line {vent_line:?}");
            let x = vent_line.x1;
            for y in int_range_from_numbers(&vent_line.y1, &vent_line.y2) {
                // println!("increasing value at ({x},{y})({})", *map.get(&(x, y))).unwrap_or(&0));
                map.entry((x, y))
                    .and_modify(|e| *e+=1)
                    .or_insert(1);
            }
        } else if vent_line.y1 == vent_line.y2 { // horizontal
            // println!("horizontal vent line {vent_line:?}");
            let y = vent_line.y1;
            for x in int_range_from_numbers(&vent_line.x1, &vent_line.x2) {
                // println!("increasing value at ({x},{y})({})", *map.get(&(x, y)).unwrap_or(&0));
                map.entry((x, y))
                    .and_modify(|e| *e+=1)
                    .or_insert(1);
            }
        }
    }

    let result = map.values().filter(|&v| *v >= 2).count();

    println!("Number of >= locations (A result): {result}");
}

fn b() {
    let mut map: HashMap<(i32, i32), i32>= HashMap::new();

    let lines = input_file_lines();

    for line in lines {
        let vent_line = VentLine::read_line(line.expect("no line"));

        if vent_line.x1 == vent_line.x2 { // vertical
            // println!("vertical vent line {vent_line:?}");
            let x = vent_line.x1;
            for y in int_range_from_numbers(&vent_line.y1, &vent_line.y2) {
                // println!("increasing value at ({x},{y})({})", *map.get(&(x, y))).unwrap_or(&0));
                map.entry((x, y))
                    .and_modify(|e| *e+=1)
                    .or_insert(1);
            }
        } else if vent_line.y1 == vent_line.y2 { // horizontal
            // println!("horizontal vent line {vent_line:?}");
            let y = vent_line.y1;
            for x in int_range_from_numbers(&vent_line.x1, &vent_line.x2) {
                // println!("increasing value at ({x},{y})({})", *map.get(&(x, y)).unwrap_or(&0));
                map.entry((x, y))
                    .and_modify(|e| *e+=1)
                    .or_insert(1);
            }
        } else if (vent_line.x1 - vent_line.x2).abs() == (vent_line.y1 - vent_line.y2).abs() {
            for (x,y) in (int_range_from_numbers(&vent_line.x1, &vent_line.x2)).into_iter()
                    .zip(int_range_from_numbers(&vent_line.y1, &vent_line.y2)) {
                map.entry((x, y))
                    .and_modify(|e| *e+=1)
                    .or_insert(1);
            }
        }
    }

    let result = map.values().filter(|&v| *v >= 2).count();

    println!("Number of >= locations (B result): {result}");
}

fn main() {
    a();
    b();
}
