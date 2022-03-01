# N-queens
> Find a solution to the N-queens puzzle using simulated annealing

Here is a solution to the 8-queens puzzle
```
- - Q - - - - -
- - - - Q - - -
- Q - - - - - -
- - - - - - - Q
- - - - - Q - -
- - - Q - - - -
- - - - - - Q -
Q - - - - - - -
```

## What is the N-queens puzzle ?
The N-queens puzzle is the problem of placing N chess queens on an NxN
chessboard so that no two queens threaten each other.

https://en.wikipedia.org/wiki/Eight_queens_puzzle

## Requirements
- cargo (You only need rustc but it is more convenient to use cargo to get
  depenencies)

## How to build
```
cargo build --release
```

## How to run
The following command runs the program with 8 queens and
a maximum of 10000 iterations
```
./target/release/nqueens 8 10000
```

You can also specify the initial temperature by using the `-t` flag.
```
./target/release/nqueens 8 10000 -t 4000
```
