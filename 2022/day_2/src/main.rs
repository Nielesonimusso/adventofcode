use common::input_file_lines;

#[derive(Debug)]
struct Position {
    position: i32,
    depth: i32,
}

fn a() {
    let mut pos = Position {
        position: 0,
        depth: 0,
    };

    let lines = input_file_lines();

    for line in lines {
        if let Ok(l) = line {
            if let Some(split) = l.split_once(" ") {
                let command = split.0;
                if let Ok(argument) = split.1.trim().parse::<i32>() {
                    match command {
                        "forward" => pos.position+=argument,
                        "down" => pos.depth+=argument,
                        "up" => pos.depth-=argument,
                        _ => (),
                    }
                }
            }
        } else {
            continue;
        }
    }
    let result = pos.position * pos.depth;
    println!("Final result A: {pos:?} = {result}");
}

fn b() {
    let mut pos = Position {
        position: 0,
        depth: 0,
    };

    let mut aim = 0;

    let lines = input_file_lines();

    for line in lines {
        if let Ok(l) = line {
            if let Some(split) = l.split_once(" ") {
                let command = split.0;
                if let Ok(argument) = split.1.trim().parse::<i32>() {
                    match command {
                        "forward" => {
                            pos.position+=argument;
                            pos.depth+=argument*aim;
                        },
                        "down" => aim+=argument,
                        "up" => aim-=argument,
                        _ => (),
                    }
                }
            }
        } else {
            continue;
        }
    }
    let result = pos.position * pos.depth;
    println!("Final result B: {pos:?} = {result}");
}

fn main() {
    a();
    b();
}
