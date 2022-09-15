mod display;
mod sudoku;

use clap::Parser;
use rand::prelude::*;
use std::fs;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct CliArgs {
    #[clap(
        short,
        long,
        help = "Generate a random seed Sudoku board to solve",
        default_value = "false"
    )]
    pub generate: bool,

    #[clap(
        short,
        long,
        help = "Load Sudoku board from specified file (see 'example.board')",
        default_value = ""
    )]
    pub input: String,
}

impl CliArgs {
    fn board_from_file(&self) -> Vec<u8> {
        let raw_file = fs::read_to_string(self.input.clone())
            .expect("failed to load file specified in --input arg");
        let board = raw_file
            .chars()
            .filter(|ch| !ch.is_whitespace())
            .collect::<String>();

        board.as_bytes().to_vec()
    }

    fn generate_board(&self) -> Vec<u8> {
        let mut r = rand::thread_rng();
        let mut board = std::iter::repeat(".").take(81).collect::<String>();
        let mut board = board.as_bytes().to_vec();

        (0..12).map({
            |_| loop {
                let mut cell_value = r.gen_range(1..=sudoku::BOARD_SIZE) as u8 + ('0' as u8);
                let mut candidate_x = r.gen_range(0..sudoku::BOARD_SIZE) as usize;
                let mut candidate_y = r.gen_range(0..sudoku::BOARD_SIZE) as usize;

                if self.valid_placement(&board, candidate_x, candidate_y, cell_value) {
                    board[(candidate_y * sudoku::BOARD_SIZE) + candidate_x] = cell_value;
                    break;
                }
            }
        });

        board
    }

    fn valid_placement(&self, board: &Vec<u8>, x: usize, y: usize, val: u8) -> bool {
        // validate candidate distinct in row
        for check_x in 0..sudoku::BOARD_SIZE {
            if x != check_x {
                if board[(y * sudoku::BOARD_SIZE) + check_x] == val {
                    return false;
                }
            }
        }

        // validate candidate distinct in col
        for check_y in 0..sudoku::BOARD_SIZE {
            if y != check_y {
                if board[(check_y * sudoku::BOARD_SIZE) + x] == val {
                    return false;
                }
            }
        }

        // validate distinct in cube
        let top = y % 3;
        let left = x % 3;
        for check_y in top..(top + 3) {
            for check_x in left..(left + 3) {
                if check_y != x && check_x != x {
                    if board[(check_y * sudoku::BOARD_SIZE) + check_x] == val {
                        return false;
                    }
                }
            }
        }

        true
    }
}

fn main() {
    let args = CliArgs::parse();

    let cfg = z3::Config::new();
    let ctx = z3::Context::new(&cfg);
    let input = "
        53..7....
        6..195...
        .98....6.
        8...6...3
        4..8.3..1
        7...2...6
        .6....28.
        ...419..5
        ....8..79
    ";
    println!("Input board:");
    println!("{}", display::from_input(&input));
    println!();

    let board = sudoku::Model::new(&ctx, &input);
    match sudoku::solve(&board) {
        Some(model) => {
            let solution = display::from_model(&board, &model);
            println!(
                "{}Z3 Solver result: SAT{}",
                display::GREEN_FG,
                display::CLEAR
            );
            println!("{}", solution);
        }
        None => println!(
            "{}Z3 Solver result: UNSAT{}",
            display::RED_FG,
            display::CLEAR
        ),
    }
    println!();
}
