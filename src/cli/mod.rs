use crate::sudoku::BOARD_SIZE;
use clap::Parser;
use rand::prelude::*;
use std::fs;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(
        short,
        long,
        help = "Generate an input Sudoku board to solve; arg is number of seed values to apply",
        default_value = "14"
    )]
    pub generate: usize,

    #[clap(
        short,
        long,
        help = "Load Sudoku board from specified file (see 'example.board')",
        default_value = ""
    )]
    pub input: String,
}

impl Args {
    pub fn board_from_file(&self) -> Vec<u8> {
        let raw_file = fs::read_to_string(self.input.clone())
            .expect("failed to load file specified in --input arg");
        let board = raw_file
            .chars()
            .filter(|ch| !ch.is_whitespace())
            .collect::<String>();

        let board = board.as_bytes();
        assert_eq!(81, board.len());
        board.to_vec()
    }

    pub fn generate_board(&self) -> Vec<u8> {
        let mut r = rand::thread_rng();

        let board = std::iter::repeat(".").take(81).collect::<String>();
        let board = board.as_bytes();
        assert_eq!(81, board.len());
        let mut board = board.to_vec();

        for _ in 0..=self.generate {
            loop {
                let cell_value = r.gen_range(1..=BOARD_SIZE) as u8 + ('0' as u8);
                let candidate_x = r.gen_range(0..BOARD_SIZE) as usize;
                let candidate_y = r.gen_range(0..BOARD_SIZE) as usize;

                if self.valid_placement(&board, candidate_x, candidate_y, cell_value) {
                    board[(candidate_y * BOARD_SIZE) + candidate_x] = cell_value;
                    break;
                }
            }
        }

        board
    }

    fn valid_placement(&self, board: &Vec<u8>, x: usize, y: usize, val: u8) -> bool {
        // validate candidate distinct in row
        for check_x in 0..BOARD_SIZE {
            if x != check_x {
                if board[(y * BOARD_SIZE) + check_x] == val {
                    return false;
                }
            }
        }

        // validate candidate distinct in col
        for check_y in 0..BOARD_SIZE {
            if y != check_y {
                if board[(check_y * BOARD_SIZE) + x] == val {
                    return false;
                }
            }
        }

        // validate distinct in cube
        let top = (y / 3) * 3;
        let left = (x / 3) * 3;
        for check_y in top..(top + 3) {
            for check_x in left..(left + 3) {
                if check_y != x && check_x != x {
                    if board[(check_y * BOARD_SIZE) + check_x] == val {
                        return false;
                    }
                }
            }
        }

        true
    }
}
