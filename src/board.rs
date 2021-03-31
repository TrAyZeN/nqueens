//! Core logic

use crate::utils::unsigned_diff;
use rand::prelude::*;
use std::cmp::Ordering;

/// A squared board containing queens
#[derive(Debug)]
pub struct Board {
    /// Size of the board
    size: usize,
    /// Sorted vector of queens by row major order
    queens: Vec<Queen>,
}

impl Board {
    /// Creates a new empty board.
    #[inline]
    #[must_use]
    pub fn new(size: usize) -> Self {
        debug_assert!(size > 0);

        Self {
            size,
            queens: Vec::new(),
        }
    }

    /// Local search a configuration
    #[must_use]
    pub fn simulated_annealing(
        &self,
        initial_temperature: f32,
        num_iterations: usize,
    ) -> Vec<Queen> {
        let mut rng = rand::thread_rng();
        let mut state = self.random_state();
        let mut e_current = self.objective(&state);

        let mut t = initial_temperature;
        let mut i = 0;
        while i < num_iterations && e_current < 0. {
            let neighbour_state = self.random_neighbour(&state);

            let e_next = self.objective(&neighbour_state);
            if e_current > e_next {
                state = neighbour_state;
                e_current = e_next;
            } else if acceptance_probability(e_current, e_next, t) >= rng.gen::<f32>() {
                state = neighbour_state;
                e_current = e_next;
            }

            t = initial_temperature / (i + 1) as f32;
            i += 1;
        }

        return state;
    }

    /// Places `self.size` queens randomly on the board
    fn random_state(&self) -> Vec<Queen> {
        let mut rng = rand::thread_rng();
        let mut state: Vec<Queen> = Vec::with_capacity(self.size);

        for _ in 0..self.size {
            self.insert_new(Queen::random(&mut rng, self.size), &mut state);
        }

        state
    }

    fn insert_new(&self, mut queen: Queen, state: &mut Vec<Queen>) -> usize {
        // We look for the insertion index because we want to maintain our
        // vector sorted
        // Note: We could use binary search here
        let mut i = 0;
        while i < state.len() && queen > state[i] {
            i += 1;
        }

        // We check if the queen already exists if not we can insert it and
        // continue
        if i == state.len() || queen != state[i] {
            state.insert(i, queen);
            return i;
        }

        let first_index = i;

        debug_assert_eq!(queen, state[i]);

        // We increment the queen position index and try to find one
        // that does not exists
        while i < state.len() - 1 && queen == state[i] {
            queen.increment_position_index(1, self.size);
            i += 1;
        }

        if i < state.len() - 1 || queen != state[i] {
            state.insert(i, queen);
            return i;
        } else if queen.get_position_index(self.size) < self.size * self.size - 1 {
            queen.increment_position_index(1, self.size);
            state.push(queen);
            return i;
        }

        debug_assert_eq!(
            queen.get_position_index(self.size),
            self.size * self.size - 1
        );

        // We have reached the end now we have to check the beginning
        queen = Queen::new(0, 0);
        i = 0;
        while i < first_index && queen == state[i] {
            queen.increment_position_index(1, self.size);
            i += 1;
        }

        state.insert(i, queen);
        return i;
    }

    /// Generates a new random neighbour of the current configuration
    /// which is the same configuration but one queen moved
    #[must_use]
    fn random_neighbour(&self, state: &Vec<Queen>) -> Vec<Queen> {
        let mut rng = rand::thread_rng();
        let mut neighbour_state = state.clone();

        let new_queen = self.insert_new(Queen::random(&mut rng, self.size), &mut neighbour_state);

        let n = match rng.gen::<usize>() % neighbour_state.len() {
            n if n == new_queen => (n + 1) % neighbour_state.len(),
            n => n,
        };
        neighbour_state.remove(n);

        neighbour_state
    }

    /// Computes number of pairs of endangered queens
    #[must_use]
    pub fn objective(&self, state: &Vec<Queen>) -> f32 {
        let mut o = 0f32;

        for i in 0..self.size {
            for j in 0..i {
                if state[j].x > state[i].x
                    && unsigned_diff(state[j].x, state[i].x)
                        == unsigned_diff(state[j].y, state[i].y)
                {
                    o -= 1f32;
                }
            }

            if i < self.size - 1 {
                for j in (i + 1)..self.size {
                    if state[j].y == state[i].y
                        || (state[j].x > state[i].x
                            && unsigned_diff(state[j].x, state[i].x)
                                == unsigned_diff(state[j].y, state[i].y))
                        || state[j].x == state[i].x
                    {
                        o -= 1f32;
                    }
                }
            }
        }

        o
    }
}

/// A structure encapsulating the position of the queen
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Queen {
    x: usize,
    y: usize,
}

impl Queen {
    /// Creates a new queen at the given position
    #[inline]
    #[must_use]
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    /// Creates a queen with random position in a square of the given size
    #[must_use]
    pub fn random(rng: &mut ThreadRng, size: usize) -> Self {
        Self {
            x: rng.gen::<usize>() % size,
            y: rng.gen::<usize>() % size,
        }
    }

    /// Increments the row major order of the queen and updates its position
    #[inline]
    pub fn increment_position_index(&mut self, increment: usize, size: usize) {
        let p = self.get_position_index(size) + increment;
        debug_assert!(p < size * size);

        self.x = p % size;
        self.y = (p / size) % size;
    }

    /// Computes the row major order of the queen
    #[inline]
    #[must_use]
    pub fn get_position_index(&self, size: usize) -> usize {
        self.x + self.y * size
    }

    /// Checks if the queen is at the given position on the board
    #[inline]
    #[must_use]
    pub fn is_at_position(&self, x: usize, y: usize) -> bool {
        x == self.x && y == self.y
    }
}

impl PartialOrd for Queen {
    #[inline]
    #[must_use]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Queen {
    #[must_use]
    fn cmp(&self, other: &Self) -> Ordering {
        match self.y.cmp(&other.y) {
            Ordering::Equal => self.x.cmp(&other.x),
            o => o,
        }
    }
}

/// Computes the acceptance probability of the next state which have less
/// energy than the current one.
#[inline]
#[must_use]
fn acceptance_probability(energy: f32, energy_next: f32, temperature: f32) -> f32 {
    debug_assert!(temperature != 0.);

    f32::exp((energy_next - energy) / temperature)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn objective() {
        let b = Board::new(4);
        let state = vec![
            Queen::new(0, 0),
            Queen::new(2, 0),
            Queen::new(1, 1),
            Queen::new(0, 2),
        ];

        assert_eq!(b.objective(&state), -6f32);
    }

    #[test]
    fn objective_2() {
        let b = Board::new(4);
        let state = vec![
            Queen::new(1, 0),
            Queen::new(3, 0),
            Queen::new(0, 2),
            Queen::new(2, 3),
        ];

        assert_eq!(b.objective(&state), -1f32);
    }

    #[test]
    fn random_state() {
        let b = Board::new(4);
        let state = b.random_state();

        for i in 1..state.len() {
            println!("{:?} {:?}", state[i - 1], state[i]);
            assert!(state[i - 1] < state[i]);
        }
    }

    #[test]
    fn random_neighbour() {
        let b = Board::new(4);
        let state = vec![
            Queen::new(1, 0),
            Queen::new(3, 0),
            Queen::new(0, 2),
            Queen::new(2, 3),
        ];
        let neighbour = b.random_neighbour(&state);

        assert_ne!(state, neighbour);

        for i in 1..neighbour.len() {
            println!("{:?} {:?}", state[i - 1], state[i]);
            assert!(neighbour[i - 1] < neighbour[i]);
        }
    }
}
