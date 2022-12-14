## Z3 Sudoku Solver + Rust
This is just a toy to play with the [Rust bindings](https://github.com/prove-rs/z3.rs) for the [Z3 SMT Solver](http://theory.stanford.edu/~nikolaj/programmingz3.html).

Quick and dirty examples:
```
# From checkout root:

# Generate a board with random seed values and if the
# solver is SATisfied, present a legal solution.
# Note: can generate UNSAT boards that appear legal; retry if needed
$ cargo r

# Solve a known input board from simple text format (whitespace is ignored)
$ cargo r -- --input example.board

# Generate a board to solve with N random seed values.
# Note can generate UNSAT boards that appear legal; retry if needed
$ cargo r -- --generate 16
```

Screenshot:

<img width="726" alt="Screen Shot 2022-09-18 at 4 47 19 PM" src="https://user-images.githubusercontent.com/32776521/190933310-c8877bb7-300a-4974-a1d4-a270372ca58a.png">
