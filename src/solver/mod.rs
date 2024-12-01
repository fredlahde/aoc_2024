use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

use anyhow::{bail, Context, Result};

use crate::aoc_api::Day;

#[allow(unused)]
pub enum Solution {
    NotSolvedYet,
    Solved(i128),
}

pub trait Solver {
    fn parse(&mut self, input: &str);
    fn solve_p1(&self) -> Result<Solution>;
    fn solve_p2(&self) -> Result<Solution>;
}

pub enum RunMode {
    Sample,
    Input,
}

pub fn run_solver_for_day(day: Day, run_mode: RunMode) -> anyhow::Result<()> {
    let mut solvers = solvers_for_days();
    let solver = solvers
        .get_mut(&day)
        .context("no solver registered for day")?;

    let input = match run_mode {
        RunMode::Sample => panic!("running against samples is not supported yet"),
        RunMode::Input => {
            let input_path = day.input_path();
            let mut fd = File::open(input_path).context("failed to open file for puzzle input")?;
            let mut input = String::new();
            fd.read_to_string(&mut input)
                .context("failed to read puzzle input")?;
            input
        }
    };

    solver.parse(&input);
    match solver.solve_p1() {
        Ok(Solution::Solved(solution)) => {
            println!("Solution for day {day} part 1 is: {solution}")
        }
        Ok(Solution::NotSolvedYet) => {
            println!("No solutioon for day {day} part 1 yet");
        }
        Err(e) => {
            bail!("failed to solve part 1: {e}")
        }
    }

    solver.parse(&input);
    match solver.solve_p2() {
        Ok(Solution::Solved(solution)) => {
            println!("Solution for day {day} part 2 is: {solution}")
        }
        Ok(Solution::NotSolvedYet) => {
            println!("No solutioon for day {day} part 2 yet");
        }
        Err(e) => {
            bail!("failed to solve part 2: {e}")
        }
    }
    Ok(())
}

mod day01;

#[allow(clippy::default_constructed_unit_structs)]
fn solvers_for_days() -> HashMap<Day, Box<dyn Solver>> {
    let mut solvers_for_days: HashMap<Day, Box<dyn Solver>> = HashMap::new();
    solvers_for_days.insert(Day::new(1), Box::new(day01::Day01::default()));
    solvers_for_days
}
