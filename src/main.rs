use std::iter::FromIterator;
use clap::Clap;

use nqueens::board::Board;

/// Finds a configuration with N queens on NxN
/// chessboard such that no queen are endangered
/// using simulated annealing.
#[derive(Clap)]
struct Opts {
    /// Number of queens on the chessboard.
    n: usize,
    /// Number of iterations.
    iterations: usize,
}

fn main() {
    let o = Opts::parse();

    // TODO: Check non zero

    let b = Board::new(o.n);
    let solution = b.simulated_annealing(1., o.iterations);
    println!("{:?}", solution);

    let mut i = 0;
    for y in 0..o.n {
        let mut line = vec!['#'; o.n];

        for x in 0..o.n {
            if i < solution.len()
                && solution[i].is_at_position(x, y) {
                line[i] = 'o';
                i += 1;
            }
        }

        println!("{}", String::from_iter(line));
    }
}

