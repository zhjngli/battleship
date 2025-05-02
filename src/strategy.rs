use rand::Rng;
use std::collections::{HashMap, HashSet};

use crate::battleship::{Battleship, BattleshipBoard};

pub trait Strategy<T> {
    fn solve(input: T) -> usize;
}

pub struct Loop;

impl Strategy<BattleshipBoard> for Loop {
    fn solve(mut bb: BattleshipBoard) -> usize {
        let mut hit = false;
        for r in 0..bb.n {
            for c in 0..bb.n {
                if bb.is_hit(r, c) {
                    hit = true;
                    break;
                }
            }
            if hit {
                break;
            }
        }

        bb.guesses
    }
}

pub struct Random;

impl Strategy<BattleshipBoard> for Random {
    fn solve(mut bb: BattleshipBoard) -> usize {
        let mut attempts = HashSet::new();
        let mut rng = rand::rng();

        loop {
            let r = rng.random_range(0..bb.n);
            let c = rng.random_range(0..bb.n);
            if attempts.contains(&(r, c)) {
                continue;
            }
            attempts.insert((r, c));

            if bb.is_hit(r, c) {
                break;
            }
        }

        bb.guesses
    }
}

pub struct MaxReduceRemaining;

impl Strategy<BattleshipBoard> for MaxReduceRemaining {
    fn solve(mut bb: BattleshipBoard) -> usize {
        // create set of all possible battleships
        let mut possible_battleships = HashSet::new();
        // add all horizontal battleships
        for r in 0..bb.n {
            for c in 1..bb.n - 1 {
                let mut battleship = HashSet::new();
                battleship.insert((r, c));
                battleship.insert((r, c - 1));
                battleship.insert((r, c + 1));
                possible_battleships.insert(Battleship { battleship });
            }
        }
        // add all vertical battleships
        for r in 1..bb.n - 1 {
            for c in 0..bb.n {
                let mut battleship = HashSet::new();
                battleship.insert((r, c));
                battleship.insert((r - 1, c));
                battleship.insert((r + 1, c));
                possible_battleships.insert(Battleship { battleship });
            }
        }

        let mut attempts = HashSet::new();
        // loop until you get a hit
        // for every location L, get the max number of possible battleships that are removed if L is not a hit
        loop {
            let mut map = HashMap::new();
            for r in 0..bb.n {
                for c in 0..bb.n {
                    map.insert((r, c), Vec::<Battleship>::new());
                }
            }

            for r in 0..bb.n {
                for c in 0..bb.n {
                    for b in &possible_battleships {
                        if b.battleship.contains(&(r, c)) {
                            map.get_mut(&(r, c)).unwrap().push(b.clone());
                        }
                    }
                }
            }

            // find the location with the most battleships
            let mut max = 0;
            let mut loc = (0, 0);
            for (&l, battleships) in &map {
                if battleships.len() > max {
                    max = battleships.len();
                    loc = l;
                }
            }
            if attempts.contains(&loc) {
                continue;
            }
            attempts.insert(loc);

            if bb.is_hit(loc.0, loc.1) {
                break;
            } else {
                // remove all battleships that contain the missed location
                for b in map.get(&loc).unwrap() {
                    possible_battleships.remove(b);
                }
            }
        }

        bb.guesses
    }
}
