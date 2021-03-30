use std::cmp::Ordering;
use crate::utils::unsigned_diff;
use rand::prelude::*;

/// A squared board containing queens
#[derive(Debug)]
pub struct Board {
    /// Size of the board
    size: usize,
    /// Sorted vector of queens by position index
    queens: Vec<Queen>,
}

impl Board {
    /// Creates a new empty board.
    #[inline]
    #[must_use]
    pub fn new(size: usize) -> Self {
        Self {
            size,
            queens: Vec::new(),
        }
    }

    /// Local search a configuration
    #[must_use]
    pub fn simulated_annealing(&self, initial_temperature: f32, num_iterations: usize) -> Vec<Queen> {
        let mut rng = rand::thread_rng();
        let mut state = self.random_state();
        let mut e_current = self.objective(&state);

        let mut t = initial_temperature;
        let mut i = 0;
        while i < num_iterations && e_current < 0. {
            let neighbour_state = self.random_neighbour(&state);

            let e_next = self.objective(&neighbour_state);
            if e_current > e_next  {
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

    /// Places `self.size` queens randomly on the board.
    fn random_state(&self) -> Vec<Queen> {
        let mut state: Vec<Queen> = Vec::with_capacity(self.size);

        let mut rng = rand::thread_rng();
        for n in 0..self.size {
            let mut i = rng.gen::<usize>() % (self.size * self.size);

            let mut j = 0;
            loop {
                while j < n && state[j].get_position_index(self.size) < i {
                    j += 1;
                }

                if j == n {
                    state.insert(j, Queen::new(i % self.size, i / self.size));
                    break;
                }

                if state[j].get_position_index(self.size) != i {
                    state.insert(j + 1, Queen::new(i % self.size, i / self.size));
                    break;
                }

                i += 1;
            }
        }

        state
    }

    /// Generates a new random configuration by moving only one queen
    fn random_neighbour(&self, state: &Vec<Queen>) -> Vec<Queen> {
        let mut rng = rand::thread_rng();
        let mut neighbour_state = state.clone();
        neighbour_state.remove(rng.gen::<usize>() % self.size);

        loop {
            let new_queen = Queen::random(&mut rng, self.size);

            let mut i = 0;
            while i < neighbour_state.len()
                && new_queen > neighbour_state[i] {
                i += 1;
            }

            if i == neighbour_state.len()
                || new_queen != neighbour_state[i] {
                neighbour_state.insert(i, new_queen);
                return neighbour_state;
            }
        }
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
    /// Creates a new queen
    #[inline]
    #[must_use]
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    /// Creates a queen with random position in the board
    pub fn random(rng: &mut ThreadRng, size: usize) -> Self {
        Self {
            x: rng.gen::<usize>() % size,
            y: rng.gen::<usize>() % size,
        }
    }

    /// Increments the 1D array position of the queen
    pub fn increment_position_index(&mut self, increment: usize, size: usize) {
        let p = self.get_position_index(size) + increment;

        self.x = p % size;
        self.y = (p / size) % size;
    }

    /// Returns 1D array position of the queen
    #[inline]
    #[must_use]
    pub fn get_position_index(&self, size: usize) -> usize {
        self.x + self.y * size
    }

    /// Checks if the queen is at the given position on the board
    #[inline]
    pub fn is_at_position(&self, x: usize, y: usize) -> bool {
        x == self.x && y == self.y
    }
}

impl PartialOrd for Queen {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Queen {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.y.cmp(&other.y) {
            Ordering::Equal => self.x.cmp(&other.x),
            o => o,
        }
    }
}

fn acceptance_probability(energy: f32, energy_next: f32, temperature: f32) -> f32 {
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
        let b = Board::new(8);
        let state = b.random_state();

        for i in 1..state.len() {
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
            assert!(neighbour[i - 1] < neighbour[i]);
        }
    }
}
