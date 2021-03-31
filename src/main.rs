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
    let o = Opts::parse();

    if o.n < 4 {
        println!("Please specify a number of queens greater than 4.");
        return;
    }

    if o.iterations == 0 {
        println!("Please specify a number of iterations greater than 0.");
        return;
    }

    let initial_temperature = o.temperature.unwrap_or(1000.);
    let b = Board::new(o.n);
    let solution = b.simulated_annealing(initial_temperature, o.iterations);

    let mut i = 0;
    for y in 0..o.n {
        let mut line = vec!["- "; o.n];

        for x in 0..o.n {
            if i < solution.len() && solution[i].is_at_position(x, y) {
                line[i] = "Q ";
                i += 1;
            }
        }

        println!("{}", String::from_iter(line));
    }
}
