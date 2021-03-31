use clap::Clap;
use std::iter::FromIterator;

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
    /// Initial temperature (default is 1000)
    #[clap(short, long)]
    temperature: Option<f32>,
}

fn main() {
    let options = Opts::parse();

    if options.n < 4 {
        println!("Please specify a number of queens greater than 4.");
        return;
    }

    if options.iterations == 0 {
        println!("Please specify a number of iterations greater than 0.");
        return;
    }

    let initial_temperature = options.temperature.unwrap_or(1000.);
    if initial_temperature < 0. {
        println!("Please specify a positive initial temperature.");
        return;
    }

    let board = Board::new(options.n);
    let solution = board.simulated_annealing(initial_temperature, options.iterations);

    let mut i = 0;
    for y in 0..options.n {
        let mut line = vec!["- "; options.n];

        for x in 0..options.n {
            if i < solution.len() && solution[i].is_at_position(x, y) {
                line[x] = "Q ";
                i += 1;
            }
        }

        println!("{}", String::from_iter(line));
    }
}
