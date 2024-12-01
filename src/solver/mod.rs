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
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

#[allow(clippy::default_constructed_unit_structs)]
fn solvers_for_days() -> HashMap<Day, Box<dyn Solver>> {
    let mut solvers_for_days: HashMap<Day, Box<dyn Solver>> = HashMap::new();
    solvers_for_days.insert(Day::new(1), Box::new(day01::Day01::default()));
    solvers_for_days.insert(Day::new(2), Box::new(day02::Day02::default()));
    solvers_for_days.insert(Day::new(3), Box::new(day03::Day03::default()));
    solvers_for_days.insert(Day::new(4), Box::new(day04::Day04::default()));
    solvers_for_days.insert(Day::new(5), Box::new(day05::Day05::default()));
    solvers_for_days.insert(Day::new(6), Box::new(day06::Day06::default()));
    solvers_for_days.insert(Day::new(7), Box::new(day07::Day07::default()));
    solvers_for_days.insert(Day::new(8), Box::new(day08::Day08::default()));
    solvers_for_days.insert(Day::new(9), Box::new(day09::Day09::default()));
    solvers_for_days.insert(Day::new(10), Box::new(day10::Day10::default()));
    solvers_for_days.insert(Day::new(11), Box::new(day11::Day11::default()));
    solvers_for_days.insert(Day::new(12), Box::new(day12::Day12::default()));
    solvers_for_days.insert(Day::new(13), Box::new(day13::Day13::default()));
    solvers_for_days.insert(Day::new(14), Box::new(day14::Day14::default()));
    solvers_for_days.insert(Day::new(15), Box::new(day15::Day15::default()));
    solvers_for_days.insert(Day::new(16), Box::new(day16::Day16::default()));
    solvers_for_days.insert(Day::new(17), Box::new(day17::Day17::default()));
    solvers_for_days.insert(Day::new(18), Box::new(day18::Day18::default()));
    solvers_for_days.insert(Day::new(19), Box::new(day19::Day19::default()));
    solvers_for_days.insert(Day::new(20), Box::new(day20::Day20::default()));
    solvers_for_days.insert(Day::new(21), Box::new(day21::Day21::default()));
    solvers_for_days.insert(Day::new(22), Box::new(day22::Day22::default()));
    solvers_for_days.insert(Day::new(23), Box::new(day23::Day23::default()));
    solvers_for_days.insert(Day::new(24), Box::new(day24::Day24::default()));
    solvers_for_days.insert(Day::new(25), Box::new(day25::Day25::default()));
    solvers_for_days
}
