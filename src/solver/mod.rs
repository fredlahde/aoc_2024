use std::collections::HashMap;

use anyhow::Result;

use crate::aoc_api::Day;

#[allow(unused)]
pub enum Solution {
    NotSolvedYet,
    Solved(i128),
}

pub trait Solver {
    fn solve_p1(&self, intput: &str) -> Result<Solution>;
    fn solve_p2(&self, intput: &str) -> Result<Solution>;
}

pub fn run_solver_for_day(day: Day, run_on_input: bool) -> anyhow::Result<()> {
    Ok(())
}

mod day01;

fn solvers_for_days() -> HashMap<Day, Box<dyn Solver>> {
    let mut solvers_for_days: HashMap<Day, Box<dyn Solver>> = HashMap::new();
    solvers_for_days.insert(Day::new(1), Box::new(day01::Day01 {}));
    solvers_for_days
}
