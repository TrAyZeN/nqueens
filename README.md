# N-queens
Solves the N-queens problem using simulated annealing.

## Requirements
- cargo (You only need rustc but it is more convenient to use cargo to get
  depenencies)

## How to build
```
cargo build
```

## How to run
The following command runs the program with 8 queens and
a maximum of 10000 iterations
```
./target/debug/nqueens 8 10000
```

You can also specify the initial temperature by using the `-t` flag.
```
./target/debug/nqueens 8 10000 -t 4000
```

