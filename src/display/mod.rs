use crate::sudoku::{Model, Pos, BOARD_SIZE};

pub const CLEAR: &str = "\x1b[0;0m";
pub const RED: &str = "\x1b[1;43m";
pub const RED_FG: &str = "\x1b[31m";
pub const GREEN: &str = "\x1b[1;42m";
pub const GREEN_FG: &str = "\x1b[32m";
pub const BLUE_FG: &str = "\x1b[0;44m";
pub const WHITE: &str = "\x1b[1;37m";

fn horizontal() -> String {
    format!("{}{}{}", WHITE, "+---------+---------+---------+\n", CLEAR)
}

fn vertical() -> String {
    format!("{}{}{}", WHITE, "|", CLEAR)
}

pub fn from_input(input: &Vec<u8>) -> String {
    let mut out = String::new();
    for y in 0..BOARD_SIZE {
        if y % 3 == 0 {
            out.push_str(&horizontal());
        }
        for x in 0..BOARD_SIZE {
            if x % 3 == 0 {
                out.push_str(&vertical());
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
        out.push_str(&vertical());
        out.push_str("\n");
    }
    out.push_str(&horizontal());

    out
}

pub fn from_model<'ctx>(model: &'ctx Model, solution: &'ctx z3::Model) -> String {
    let mut out = String::new();
    for y in 0..BOARD_SIZE {
        if y % 3 == 0 {
            out.push_str(&horizontal());
        }

        for x in 0..BOARD_SIZE {
            if x % 3 == 0 {
                out.push_str(&vertical());
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
        out.push_str(&vertical());
        out.push_str("\n");
    }
    out.push_str(&horizontal());

    out
}
