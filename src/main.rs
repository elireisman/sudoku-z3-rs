use std::collections::{HashMap, HashSet};

const BOARD_SIZE: usize = 9;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy)]
struct Pos {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone)]
struct Cell<'ctx> {
    value: z3::ast::Int<'ctx>,
}

impl<'ctx> Cell<'ctx> {
    fn default(ctx: &'ctx z3::Context, pos: Pos) -> Self {
        Cell {
            value: z3::ast::Int::fresh_const(ctx, format!("value_x{}_y{}", pos.x, pos.y).as_str()),
        }
    }

    fn apply_constraints(&self, ctx: &'ctx z3::Context, solver: &'ctx z3::Solver) {
        // invariant: cell value >= 1
        solver.assert(&self.value.ge(&z3::ast::Int::from_i64(ctx, 1)));
        // invariant: cell value <= 9
        solver.assert(&self.value.le(&z3::ast::Int::from_i64(ctx, 9)));
    }
}

type Sudoku<'ctx> = HashMap<Pos, Cell<'ctx>>;

#[derive(Debug, Clone)]
struct SudokuModel<'ctx> {
    ctx: &'ctx z3::Context,
    board: Sudoku<'ctx>,
}

impl<'ctx> SudokuModel<'ctx> {
    fn apply_constraints(&self, solver: &'ctx z3::Solver) {
        self.board
            .values()
            .for_each(|cell| cell.apply_constraints(self.ctx, solver));
        self.constrain_rows(solver);
        self.constrain_cols(solver);
        self.constrain_cubes(solver);
    }

    fn constrain_rows(&self, solver: &'ctx z3::Solver) {
        for y in 0..BOARD_SIZE {
            let mut row = HashSet::<i64>::new();
            for x in 0..BOARD_SIZE {
                row.insert(
                    self.board
                        .get(&Pos { x, y })
                        .unwrap()
                        .value
                        .as_i64()
                        .unwrap_or(0),
                );
            }

            for expected in 1..=BOARD_SIZE {
                let expected = expected as i64;
                solver.assert(&z3::ast::Bool::from_bool(self.ctx, row.contains(&expected)));
                row.remove(&expected);
            }
            solver.assert(&z3::ast::Bool::from_bool(self.ctx, row.is_empty()));
        }
    }

    fn constrain_cols(&self, solver: &z3::Solver) {
        for x in 0..BOARD_SIZE {
            let mut col = HashSet::<i64>::new();
            for y in 0..BOARD_SIZE {
                col.insert(
                    self.board
                        .get(&Pos { x, y })
                        .unwrap()
                        .value
                        .as_i64()
                        .unwrap_or(0),
                );
            }

            for expected in 1..=BOARD_SIZE {
                let expected = expected as i64;
                solver.assert(&z3::ast::Bool::from_bool(self.ctx, col.contains(&expected)));
                col.remove(&expected);
            }
            solver.assert(&z3::ast::Bool::from_bool(self.ctx, col.is_empty()));
        }
    }

    fn constrain_cubes(&self, solver: &z3::Solver) {
        for pos in vec![
            Pos { x: 0, y: 0 },
            Pos { x: 0, y: 3 },
            Pos { x: 0, y: 6 },
            Pos { x: 3, y: 0 },
            Pos { x: 6, y: 0 },
            Pos { x: 3, y: 3 },
            Pos { x: 3, y: 6 },
            Pos { x: 6, y: 3 },
            Pos { x: 6, y: 6 },
        ] {
            self.constrain_cube(pos, solver);
        }
    }

    fn constrain_cube(&self, top_left: Pos, solver: &z3::Solver) {
        for y in top_left.y..(top_left.y + 3) {
            let mut cube = HashSet::<i64>::new();
            for x in top_left.x..(top_left.x + 3) {
                cube.insert(
                    self.board
                        .get(&Pos { x, y })
                        .unwrap()
                        .value
                        .as_i64()
                        .unwrap_or(0),
                );
            }

            for expected in 1..=BOARD_SIZE {
                let expected = expected as i64;
                solver.assert(&z3::ast::Bool::from_bool(
                    self.ctx,
                    cube.contains(&expected),
                ));
                cube.remove(&expected);
            }
            solver.assert(&z3::ast::Bool::from_bool(self.ctx, cube.is_empty()));
        }
    }
}

fn main() {
    let cfg = z3::Config::new();
    let ctx = z3::Context::new(&cfg);

    let mut board = HashMap::<Pos, Cell>::new();
    for y in 0..BOARD_SIZE {
        for x in 0..BOARD_SIZE {
            let pos = Pos { x, y };
            board.insert(pos, Cell::default(&ctx, pos));
        }
    }
    let model = SudokuModel {
        ctx: &ctx,
        board: board,
    };

    let solver = z3::Solver::new(&ctx);
    model.apply_constraints(&solver);
    match solver.check() {
        z3::SatResult::Sat => println!("Solution: {:?}", solver.get_model().unwrap()),
        _ => println!("Unsat!: {:?}", solver.get_proof()),
    }
}
