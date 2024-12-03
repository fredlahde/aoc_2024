use super::Solver;

pub enum Instruction {
    Mul(i128, i128),
    Do,
    Dont,
}

#[derive(Default)]
pub struct Day03 {
    instructions: Vec<Instruction>,
}

impl Solver for Day03 {
    #[allow(unused)]
    fn parse(&mut self, input: &str) {
        let re = regex::Regex::new(r#"(do\(\))|(don't\(\))|mul\((\d{1,3}),(\d{1,3})\)"#).unwrap();
        let mut sum: i128 = 0;
        let mut mul_enabled = true;
        for m in re.captures_iter(input) {
            if m.get(1).is_some() {
                self.instructions.push(Instruction::Do);
                continue;
            }
            if m.get(2).is_some() {
                self.instructions.push(Instruction::Dont);
                continue;
            }
            if let (Some(a), Some(b)) = (m.get(3), m.get(4)) {
                let a = a.as_str().parse().unwrap();
                let b = b.as_str().parse().unwrap();
                self.instructions.push(Instruction::Mul(a, b));
            }
        }
    }

    fn solve_p1(&self) -> anyhow::Result<super::Solution> {
        let answer = self
            .instructions
            .iter()
            .map(|i| match i {
                Instruction::Mul(a, b) => a * b,
                Instruction::Do | Instruction::Dont => 0,
            })
            .sum();

        Ok(super::Solution::Solved(answer))
    }

    fn solve_p2(&self) -> anyhow::Result<super::Solution> {
        let mut mul_enabled = true;
        let mut sum = 0;
        for i in &self.instructions {
            match i {
                Instruction::Mul(a, b) if mul_enabled => sum += a * b,
                Instruction::Do => mul_enabled = true,
                Instruction::Dont => mul_enabled = false,
                Instruction::Mul(_, _) => {}
            }
        }

        Ok(super::Solution::Solved(sum))
    }
}
