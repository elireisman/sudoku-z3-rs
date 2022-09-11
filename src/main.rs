use itertools::Itertools;
use std::collections::HashMap;

const BOARD_SIZE: usize = 9;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
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
            let row_cells = self.get_row(y);
            let pair_clauses = row_cells
                .into_iter()
                .combinations(2)
                .map(|pair| {
                    let lt = pair[0].lt(pair[1]);
                    let gt = pair[0].gt(pair[1]);
                    let clauses = vec![&lt, &gt];
                    z3::ast::Bool::or(self.ctx, &clauses)
                })
                .collect::<Vec<_>>();
            let assertions_expr =
                z3::ast::Bool::and(self.ctx, &pair_clauses.iter().collect::<Vec<_>>());
            solver.assert(&assertions_expr);
        }
    }

    fn constrain_cols(&self, solver: &z3::Solver) {
        for x in 0..BOARD_SIZE {
            let col_cells = self.get_column(x);
            let pair_clauses = col_cells
                .into_iter()
                .combinations(2)
                .map(|pair: Vec<&'ctx z3::ast::Int<'_>>| {
                    let lt = pair[0].lt(pair[1]);
                    let gt = pair[0].gt(pair[1]);
                    let clauses = vec![&lt, &gt];
                    z3::ast::Bool::or(self.ctx, &clauses)
                })
                .collect::<Vec<_>>();
            let assertions_expr =
                z3::ast::Bool::and(self.ctx, &pair_clauses.iter().collect::<Vec<_>>());
            solver.assert(&assertions_expr);
        }
    }

    fn constrain_cubes(&self, solver: &z3::Solver) {
        for cube in vec![
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
            let cube_cells = self.get_cube(cube);
            let pair_clauses = cube_cells
                .into_iter()
                .combinations(2)
                .map(|pair: Vec<&'ctx z3::ast::Int<'_>>| {
                    let lt = pair[0].lt(pair[1]);
                    let gt = pair[0].gt(pair[1]);
                    let clauses = vec![&lt, &gt];
                    z3::ast::Bool::or(self.ctx, &clauses)
                })
                .collect::<Vec<_>>();
            let assertions_expr =
                z3::ast::Bool::and(self.ctx, &pair_clauses.iter().collect::<Vec<_>>());
            solver.assert(&assertions_expr);
        }
    }

    fn get_cube(&self, top_left: Pos) -> Vec<&'ctx z3::ast::Int> {
        let mut cube: Vec<&'ctx z3::ast::Int> = vec![];
        for y in top_left.y..(top_left.y + 3) {
            for x in top_left.x..(top_left.x + 3) {
                let next_pos = Pos { x, y };
                cube.push(&self.board.get(&next_pos).unwrap().value);
            }
        }

        cube
    }

    // obtain a Vec of references to all row members except the target
    fn get_row(&self, target_y: usize) -> Vec<&'ctx z3::ast::Int> {
        let mut row: Vec<&'ctx z3::ast::Int> = vec![];
        for x in 0..BOARD_SIZE {
            let next_pos = Pos { x: x, y: target_y };
            row.push(&self.board.get(&next_pos).unwrap().value);
        }

        row
    }

    // obtain a Vec of references to all col members except the target
    fn get_column(&self, target_x: usize) -> Vec<&'ctx z3::ast::Int> {
        let mut col: Vec<&'ctx z3::ast::Int> = vec![];
        for y in 0..BOARD_SIZE {
            let next_pos = Pos { x: target_x, y: y };
            col.push(&self.board.get(&next_pos).unwrap().value);
        }

        col
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
