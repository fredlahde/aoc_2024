use std::collections::HashMap;

use super::Solver;

#[derive(Default)]
pub struct Day01 {
    list_a: Vec<i128>,
    list_b: Vec<i128>,
}

impl Solver for Day01 {
    #[allow(unused)]
    fn parse(&mut self, input: &str) {
        input
            .lines()
            .filter(|l| !l.is_empty())
            .map(|l| l.split_once("   ").unwrap())
            .for_each(|(a, b)| {
                self.list_a.push(a.parse().unwrap());
                self.list_b.push(b.parse().unwrap());
            });

        self.list_a.sort();
        self.list_b.sort();
    }

    fn solve_p1(&self) -> anyhow::Result<super::Solution> {
        Ok(super::Solution::Solved(
            self.list_a
                .iter()
                .zip(self.list_b.iter())
                .map(|(a, b)| (a - b).abs())
                .sum(),
        ))
    }

    fn solve_p2(&self) -> anyhow::Result<super::Solution> {
        let mut frequency_map: HashMap<i128, i128> = HashMap::new();
        for x in &self.list_b {
            *frequency_map.entry(*x).or_default() += 1;
        }

        let answer: i128 = self
            .list_a
            .iter()
            .map(|a| a * *frequency_map.get(a).unwrap_or(&0))
            .sum();
        Ok(super::Solution::Solved(answer))
    }
}
