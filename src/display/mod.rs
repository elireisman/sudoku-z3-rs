use crate::sudoku::{Model, Pos, BOARD_SIZE};

pub const CLEAR: &str = "\x1b[0;0m";
pub const RED: &str = "\x1b[1;43m";
pub const RED_FG: &str = "\x1b[31m";
pub const GREEN: &str = "\x1b[1;42m";
pub const GREEN_FG: &str = "\x1b[32m";
pub const BLUE_FG: &str = "\x1b[0;44m";
pub const WHITE: &str = "\x1b[1;37m";

const TOP_RIGHT_CORNER: &str = "\x1b(0\x6b";
const TOP_LEFT_CORNER: &str = "\x1b(0\x6c";
const RIGHT_CORNER: &str = "\x1b(0\x75";
const LEFT_CORNER: &str = "\x1b(0\x74";
const BOTTOM_RIGHT_CORNER: &str = "\x1b(0\x6a";
const BOTTOM_LEFT_CORNER: &str = "\x1b(0\x6d";
const VERTICAL: &str = "\x1b(0\x78\x1b(0";
const HORIZONTAL: &str = "\x1b(0\x71";
const TOP_SPLIT: &str = "\x1b(0\x77";
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
            TOP_SPLIT,
            &HORIZONTAL.repeat(9),
            TOP_SPLIT,
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

pub fn from_input(input: &Vec<u8>) -> String {
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

            match input[(y * BOARD_SIZE) + x] as char {
                '.' => {
                    let elem = format!("{} . {}", BLUE_FG, CLEAR);
                    out.push_str(&elem);
                }
                ch => {
                    let elem = format!("{} {} {}", RED, ch, CLEAR);
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

pub fn from_model<'ctx>(model: &'ctx Model, solution: &'ctx z3::Model) -> String {
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

            let pos = Pos::new(x, y);
            let cell = &model.board[&pos];
            let solved = solution.eval(cell.get_value(), true);
            if cell.is_solution() {
                let elem = format!("{} {} {}", RED, &solved.unwrap(), CLEAR);
                out.push_str(&elem);
            } else {
                let elem = format!("{} {} {}", GREEN, &solved.unwrap(), CLEAR);
                out.push_str(&elem);
            }
        }
        out.push_str(&VERTICAL);
        out.push_str("\n");
    }
    out.push_str(&horizontal_bottom());

    out
}
