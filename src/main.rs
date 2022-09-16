mod cli;
mod display;
mod sudoku;

use clap::Parser;

fn main() {
    let args = cli::Args::parse();
    let input: Vec<u8>;

    if args.input.len() > 0 {
        println!("Loading board from input file: {}", args.input);
        input = args.board_from_file();
    } else {
        println!("Generating random board with {} seed values", args.generate);
        input = args.generate_board();
    }
    println!("Input board:");
    println!("{}", display::from_input(&input));
    println!();

    let cfg = z3::Config::new();
    let ctx = z3::Context::new(&cfg);
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
