## Z3 Sudoku Solver + Rust
This is just a toy to play with the [Rust bindings](https://github.com/prove-rs/z3.rs) for the [Z3 SMT Solver](http://theory.stanford.edu/~nikolaj/programmingz3.html). Not all the APIs you'd expect are implemented yet, so I used what was available, and may contribute some nice things from other bindings in the future (?)

Quick and dirty examples:
```
# From checkout root:

# Solve a known input board from simple text format (whitespace is ignored)
$ cargo r -- --input example.board

# Generate a board to solve with N random seed values.
# Note this can generate legal but UNSAT boards, retry if needed
$ cargo r -- --generate 14
```

Screenshot:
<img width="658" alt="Screen Shot 2022-09-15 at 10 56 53 AM" src="https://user-images.githubusercontent.com/32776521/190476375-054cb1b4-1fd3-46a3-87c1-7574e6ceb952.png">
