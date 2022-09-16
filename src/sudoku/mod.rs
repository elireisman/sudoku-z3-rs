use itertools::Itertools;
use std::collections::HashMap;

pub const BOARD_SIZE: usize = 9;

pub fn solve<'ctx>(input: &Model<'ctx>) -> Option<z3::Model<'ctx>> {
    let solver = z3::Solver::new(input.ctx);
    input.apply_constraints(&solver);
    match solver.check() {
        z3::SatResult::Sat => {
            let model = solver.get_model().unwrap();
            Some(model)
        }
        _ => None,
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    pub fn new(x: usize, y: usize) -> Self {
        Pos { x, y }
    }
}

#[derive(Debug, Clone)]
pub struct Cell<'ctx> {
    value: z3::ast::Int<'ctx>,
    from_input: bool,
}

impl<'ctx> Cell<'ctx> {
    fn default(ctx: &'ctx z3::Context, pos: Pos) -> Self {
        // an open board position Z3 should resolve as part of the puzzle solution
        Cell {
            value: z3::ast::Int::fresh_const(ctx, format!("value_x{}_y{}", pos.x, pos.y).as_str()),
            from_input: false,
        }
    }

    // a fixed value set by the input board - Z3 shouldn't mess with this
    fn from_input(ctx: &'ctx z3::Context, value: i64) -> Self {
        Cell {
            value: z3::ast::Int::from_i64(ctx, value),
            from_input: true,
        }
    }

    pub fn get_value(&self) -> &z3::ast::Int<'ctx> {
        return &self.value;
    }

    pub fn is_solution(&self) -> bool {
        return self.from_input;
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
pub struct Model<'ctx> {
    pub ctx: &'ctx z3::Context,
    pub board: Sudoku<'ctx>,
}

impl<'ctx> Model<'ctx> {
    pub fn new(ctx: &'ctx z3::Context, input: &Vec<u8>) -> Model<'ctx> {
        let mut board = HashMap::<Pos, Cell>::new();

        for y in 0..BOARD_SIZE {
            for x in 0..BOARD_SIZE {
                let pos = Pos { x, y };
                let next_char = input[(y * BOARD_SIZE) + x] as char;
                match next_char {
                    '.' => board.insert(pos, Cell::default(&ctx, pos)),
                    ch => board.insert(pos, Cell::from_input(&ctx, ch as i64 - '0' as i64)),
                };
            }
        }

        Model {
            ctx: &ctx,
            board: board,
        }
    }

    fn apply_constraints(&self, solver: &'ctx z3::Solver) {
        // all cells in the Sudoku board must hold values in [1,9]
        self.board
            .values()
            .for_each(|cell| cell.apply_constraints(self.ctx, solver));

        // all rows of the Sudoku board must contain unique values
        self.constrain_rows(solver);
        // all columns of the Sudoku board must contain unique values
        self.constrain_cols(solver);
        // all 3x3 cubes of the Sudoku board must contain unique values
        self.constrain_cubes(solver);
    }

    fn constrain_rows(&self, solver: &'ctx z3::Solver) {
        for y in 0..BOARD_SIZE {
            let row_cells = self.get_row(y);
            self.constrain_distinct_values(row_cells, solver);
        }
    }

    fn constrain_cols(&self, solver: &z3::Solver) {
        for x in 0..BOARD_SIZE {
            let col_cells = self.get_column(x);
            self.constrain_distinct_values(col_cells, solver);
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
            self.constrain_distinct_values(cube_cells, solver);
        }
    }

    // take a selected subset of Sudoku board cells (row, col, 3x3 cube)
    // and ensure every value in the subset is constrainted in Z3 as distinct.
    fn constrain_distinct_values(&self, board_cells: Vec<&'ctx z3::ast::Int>, solver: &z3::Solver) {
        let all_pairs = board_cells
            .into_iter()
            .combinations(2)
            .map(|pair: Vec<&'ctx z3::ast::Int<'_>>| {
                let lt = pair[0].lt(pair[1]);
                let gt = pair[0].gt(pair[1]);
                let clauses = vec![&lt, &gt];
                z3::ast::Bool::or(self.ctx, &clauses)
            })
            .collect::<Vec<_>>();
        let assertions_expr = z3::ast::Bool::and(self.ctx, &all_pairs.iter().collect::<Vec<_>>());
        solver.assert(&assertions_expr);
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
