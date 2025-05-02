use rand::Rng;
use std::{collections::HashSet, hash::Hash};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Battleship {
    pub battleship: HashSet<(usize, usize)>,
}

impl Hash for Battleship {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for &p in &self.battleship {
            p.hash(state);
        }
    }
}

#[derive(Debug, Clone)]
pub struct BattleshipBoard {
    pub n: usize,
    pub guesses: usize,
    battleship: HashSet<(usize, usize)>,
}

impl BattleshipBoard {
    pub fn new(n: usize) -> Self {
        let mut battleship = HashSet::new();

        let mut rng = rand::rng();
        let vertical = rng.random_bool(0.5);
        if vertical {
            let center = (rng.random_range(1..n - 1), rng.random_range(0..n));
            let left = (center.0 - 1, center.1);
            let right = (center.0 + 1, center.1);
            battleship.insert(center);
            battleship.insert(left);
            battleship.insert(right);
        } else {
            let center = (rng.random_range(0..n), rng.random_range(1..n - 1));
            let up = (center.0, center.1 - 1);
            let down = (center.0, center.1 + 1);
            battleship.insert(center);
            battleship.insert(up);
            battleship.insert(down);
        }

        BattleshipBoard {
            n,
            battleship,
            guesses: 0,
        }
    }

    pub fn is_hit(&mut self, r: usize, c: usize) -> bool {
        self.guesses += 1;
        self.battleship.contains(&(r, c))
    }

    pub fn check_battleship(&self, battleship: &HashSet<(usize, usize)>) -> bool {
        &self.battleship == battleship
    }
}
