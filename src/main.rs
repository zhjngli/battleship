pub mod battleship;
pub mod strategy;

use battleship::BattleshipBoard;
use strategy::{Loop, MaxReduceRemaining, Random, Strategy};

fn main() {
    let total_runs = 1000;
    let mut total_guesses_loop = 0;
    let mut total_guesses_random = 0;
    let mut total_guesses_max_reduce = 0;
    for _ in 0..total_runs {
        let bb = BattleshipBoard::new(8);
        total_guesses_loop += Loop::solve(bb.clone());
        total_guesses_random += Random::solve(bb.clone());
        total_guesses_max_reduce += MaxReduceRemaining::solve(bb.clone());
    }
    println!("Total runs: {}", total_runs);
    println!(
        "Average guesses for Loop: {}",
        (total_guesses_loop as f64) / (total_runs as f64)
    );
    println!(
        "Average guesses for Random: {}",
        (total_guesses_random as f64) / (total_runs as f64)
    );
    println!(
        "Average guesses for MaxReduceRemaining: {}",
        (total_guesses_max_reduce as f64) / (total_runs as f64)
    );
}
