use std::fs::File;
use std::io::{Lines, BufReader};

use common::input_file_lines;

#[derive(Debug)]
struct Board {
    numbers: [[i32; 5]; 5],
    crossed: [[bool; 5]; 5],
}

impl Board {
    fn read_board(reader: &mut Lines<BufReader<File>>) -> Self {
        let mut numbers: [[i32; 5]; 5] = [[0; 5]; 5];
        let crossed: [[bool; 5]; 5] = [[false; 5]; 5];
        for r in 0..5 {
            let row = reader.next().expect("No board line").expect("Invalid board line");
            for (c, column) in row.split(" ").filter(|s| !s.is_empty()).enumerate() {
                numbers[r][c] = column.parse().expect("Invalid board number");
            }
        }
        Board { numbers, crossed }
    }

    fn check_column(&self, column: usize) -> bool {
        self.crossed.iter().map(|r| r[column]).all(|c| c)
    }

    fn check_row(&self, row: usize) -> bool {
        self.crossed[row].into_iter().all(|c| c)
    }
 
    fn check_diagonals(&self) -> bool {
        self.crossed.into_iter().flatten().step_by(6).all(|c| c) ||
        self.crossed.into_iter().flatten().step_by(4).all(|c| c)
    }

    fn calculate_score(&self, number: &i32) -> i32 {
        self.numbers.iter().flatten().zip(self.crossed.iter().flatten()).map(|p| if !p.1 { *p.0 } else { 0 }).sum::<i32>() * number
    }

    fn cross_off(&mut self, number: &i32) -> bool {
        for (r, row) in self.numbers.iter().enumerate() {
            for (c, n) in row.iter().enumerate() {
                if n == number {
                    self.crossed[r][c] = true;
                    return self.check_column(c) || self.check_row(r) || self.check_diagonals();
                }
            }
        }
        return false;
    }
}

fn a() {
    let mut lines = input_file_lines();

    // read first line of numbers
    let numbers = lines.next()
        .expect("no numbers")
        .expect("number error")
        .split(",")
        .map(|n| n.parse().expect("number not a number"))
        .collect::<Vec<i32>>();

    let mut boards: Vec<Board> = Vec::new();

    while lines.next().is_some() {
        boards.push(Board::read_board(&mut lines));
    }

    for (index, number) in numbers.iter().enumerate() {
        for board in &mut boards {
            if board.cross_off(number) {
                let score = board.calculate_score(number);
                println!("Part A score: {score} for board {board:?} after drawing numbers {:?}", &numbers[0..index + 1]);
                return;
            }
        }
    }
}

fn b() {
    let mut lines = input_file_lines();

    // read first line of numbers
    let numbers = lines.next()
        .expect("no numbers")
        .expect("number error")
        .split(",")
        .map(|n| n.parse().expect("number not a number"))
        .collect::<Vec<i32>>();

    let mut boards: Vec<Board> = Vec::new();

    while lines.next().is_some() {
        boards.push(Board::read_board(&mut lines));
    }

    let mut number_generator = numbers.into_iter();
    while boards.len() > 1 {
        if let Some(number) = number_generator.next() {
            boards.retain_mut(|board| !board.cross_off(&number));
        }
    }

    let final_board: &mut Board = boards.get_mut(0).expect("No board left");
    let mut number_guess = number_generator.next().expect("No number left");
    while !final_board.cross_off(&number_guess) { 
        number_guess = number_generator.next().expect("No number left"); 
    }

    let final_score = final_board.calculate_score(&number_guess);
    println!("Part B score: {final_score} for board {final_board:?}");
}

fn main() {
    a();
    b();
}
