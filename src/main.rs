mod display;
mod sudoku;

fn main() {
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
