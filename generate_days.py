#!/usr/bin/env python3

solver_template = """
use super::Solver;

#[derive(Default)]
pub struct Day%DAY {}

impl Solver for Day%DAY {
    #[allow(unused)]
    fn parse(&mut self, input: &str) {}

    fn solve_p1(&self) -> anyhow::Result<super::Solution> {
        Ok(super::Solution::NotSolvedYet)
    }

    fn solve_p2(&self) -> anyhow::Result<super::Solution> {
        Ok(super::Solution::NotSolvedYet)
    }
}
"""

def main():
    for i in range(1, 26):
        day_format = f"{i:02}";
        code = solver_template.replace("%DAY", day_format)
        with open(f"src/solver/day{day_format}.rs", "w+") as fd:
            fd.write(code)

    for i in range(1, 26):
        day_format = f"{i:02}";
        print(f"mod day{day_format};")

    print()
    for i in range(1, 26):
        day_format = f"{i:02}";
        print(f"solvers_for_days.insert(Day::new({i}), Box::new(day{day_format}::Day{day_format}::default()));")




if __name__ == "__main__":
    main()
