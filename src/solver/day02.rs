use super::Solver;

#[derive(Default)]
pub struct Day02 {
    lines: Vec<Vec<i32>>,
}

impl Solver for Day02 {
    #[allow(unused)]
    fn parse(&mut self, input: &str) {
        self.lines = input
            .lines()
            .map(|l| {
                l.split_whitespace()
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect()
            })
            .collect();
    }

    fn solve_p1(&self) -> anyhow::Result<super::Solution> {
        let mut safe_count = 0;
        for l in &self.lines {
            let safe = check_safe(l);
            if safe {
                safe_count += 1;
            }
        }

        Ok(super::Solution::Solved(safe_count))
    }

    fn solve_p2(&self) -> anyhow::Result<super::Solution> {
        let mut safe_count = 0;
        for l in &self.lines {
            if check_safe(l) {
                safe_count += 1;
                continue;
            }
            for idx in 0..l.len() {
                let mut l = l.clone();
                l.remove(idx);
                let safe = check_safe(&l);
                if safe {
                    safe_count += 1;
                    break;
                }
            }
        }

        Ok(super::Solution::Solved(safe_count))
    }
}

fn check_safe(line: &[i32]) -> bool {
    let mut safe = true;
    let mut last = None;
    let mut increase = true;
    let mut first_check = true;
    for x in line {
        if let Some(last) = last {
            if x == last {
                safe = false;
                break;
            }
            if first_check {
                increase = x > last;
                first_check = false
            } else {
                if increase {
                    if x <= last {
                        safe = false;
                        break;
                    }
                }
                if !increase {
                    if x >= last {
                        safe = false;
                        break;
                    }
                }
            }
            if x.abs_diff(*last) < 1 || x.abs_diff(*last) > 3 {
                safe = false;
                break;
            }
        }
        last = Some(x)
    }
    safe
}
