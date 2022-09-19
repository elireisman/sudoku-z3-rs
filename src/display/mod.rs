use crate::sudoku::{Model, Pos, BOARD_SIZE};
use std::collections::HashMap;

pub fn from_input(input: &Vec<u8>) -> String {
    let view = view_from_input(input);
    render(view)
}

pub fn from_model<'ctx>(model: &'ctx Model, solution: &'ctx z3::Model) -> String {
    let view = view_from_model(model, solution);
    render(view)
}

type BoardView = HashMap<usize, HashMap<usize, CellView>>;

struct CellView(u8, bool);

// pretty-print the Sudoku board, quick'n'dirty style :)
fn render(board: BoardView) -> String {
    let mut out = String::new();
    out.push_str(&horizontal_top());
    for y in 0..BOARD_SIZE {
        if y == 3 || y == 6 {
            out.push_str(&horizontal());
        }
        for x in 0..BOARD_SIZE {
            if x % 3 == 0 {
                out.push_str(&VERTICAL);
            }

            match board[&x][&y] {
                CellView(0, _) => {
                    let elem = format!("{} . {}", BLUE_FG, CLEAR);
                    out.push_str(&elem);
                }
                CellView(value, from_input) => {
                    let elem = match from_input {
                        true => format!("{} {} {}", RED, value, CLEAR),
                        false => format!("{} {} {}", GREEN, value, CLEAR),
                    };
                    out.push_str(&elem);
                }
            }
        }
        out.push_str(&VERTICAL);
        out.push_str("\n");
    }
    out.push_str(&horizontal_bottom());

    out
}

// for convenience (used in main)
pub const CLEAR: &str = "\x1b[0;0m";
pub const RED_FG: &str = "\x1b[31m";
pub const GREEN_FG: &str = "\x1b[32m";

const RED: &str = "\x1b[1;43m";
const GREEN: &str = "\x1b[1;42m";
const BLUE_FG: &str = "\x1b[0;44m";
const WHITE: &str = "\x1b[1;37m";

const TOP_RIGHT_CORNER: &str = "\x1b(0\x6b";
const TOP_LEFT_CORNER: &str = "\x1b(0\x6c";
const RIGHT_CORNER: &str = "\x1b(0\x75";
const LEFT_CORNER: &str = "\x1b(0\x74";
const BOTTOM_RIGHT_CORNER: &str = "\x1b(0\x6a";
const BOTTOM_LEFT_CORNER: &str = "\x1b(0\x6d";
const VERTICAL: &str = "\x1b(0\x78\x1b(0";
const HORIZONTAL: &str = "\x1b(0\x71";
const TOP_SPLIT: &str = "\x1b(0\x77";
const SPLIT: &str = "\x1b(0\x6e";
const BOTTOM_SPLIT: &str = "\x1b(0\x76";
const EOL: &str = "\x1b(B\n";

fn horizontal_top() -> String {
    format!(
        "{}{}{}",
        WHITE,
        vec![
            TOP_LEFT_CORNER,
            &HORIZONTAL.repeat(9),
            TOP_SPLIT,
            &HORIZONTAL.repeat(9),
            TOP_SPLIT,
            &HORIZONTAL.repeat(9),
            TOP_RIGHT_CORNER,
            EOL,
        ]
        .join(""),
        CLEAR,
    )
}

fn horizontal() -> String {
    format!(
        "{}{}{}",
        WHITE,
        vec![
            LEFT_CORNER,
            &HORIZONTAL.repeat(9),
            SPLIT,
            &HORIZONTAL.repeat(9),
            SPLIT,
            &HORIZONTAL.repeat(9),
            RIGHT_CORNER,
            EOL,
        ]
        .join(""),
        CLEAR,
    )
}

fn horizontal_bottom() -> String {
    format!(
        "{}{}{}",
        WHITE,
        vec![
            BOTTOM_LEFT_CORNER,
            &HORIZONTAL.repeat(9),
            BOTTOM_SPLIT,
            &HORIZONTAL.repeat(9),
            BOTTOM_SPLIT,
            &HORIZONTAL.repeat(9),
            BOTTOM_RIGHT_CORNER,
            EOL,
        ]
        .join(""),
        CLEAR,
    )
}

fn view_from_input(input: &Vec<u8>) -> BoardView {
    let mut out: BoardView = HashMap::new();
    for y in 0..BOARD_SIZE {
        for x in 0..BOARD_SIZE {
            let elem = match input[(y * BOARD_SIZE) + x] as char {
                '.' => CellView(0, false),
                ch => {
                    let value: u8 = (ch as u8) - ('0' as u8);
                    if value < 1 || value > 9 {
                        panic!("illegal input value: {} must be in range [1..9]", value);
                    }
                    CellView(value, false)
                }
            };

            if !out.contains_key(&x) {
                out.insert(x, HashMap::new());
            }
            out.get_mut(&x).unwrap().insert(y, elem);
        }
    }

    out
}

fn view_from_model<'ctx>(model: &'ctx Model, solution: &'ctx z3::Model) -> BoardView {
    let mut out: BoardView = HashMap::new();
    for y in 0..BOARD_SIZE {
        for x in 0..BOARD_SIZE {
            let pos = Pos::new(x, y);
            let cell = &model.board[&pos];
            let solved = solution.eval(cell.get_value(), true);
            let value = solved.unwrap().as_i64().unwrap() as u8;
            let elem = CellView(value, cell.is_from_input());

            if !out.contains_key(&x) {
                out.insert(x, HashMap::new());
            }
            out.get_mut(&x).unwrap().insert(y, elem);
        }
    }

    out
}
