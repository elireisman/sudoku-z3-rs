## Z3 Sudoku Solver + Rust
This is just a toy to play with the [Rust bindings](https://github.com/prove-rs/z3.rs) for the [Z3 SMT Solver](http://theory.stanford.edu/~nikolaj/programmingz3.html). This binding is still missing some nice-to-have APIs so I used what was available.

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


