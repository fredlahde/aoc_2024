use std::str::FromStr;

use anyhow::{bail, Context};
use aoc_2024::{
    aoc_api::{AOCClient, Day},
    solver::run_solver_for_day,
};

enum Mode {
    Download,
    Sample,
    Input,
}

impl FromStr for Mode {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "download" => Ok(Mode::Download),
            "sample" => Ok(Mode::Sample),
            "input" => Ok(Mode::Input),
            _ => bail!("invalid mode"),
        }
    }
}

fn main() -> anyhow::Result<()> {
    let config = aoc_2024::load_config()?;
    let mut args = std::env::args();
    let _program_name = args.next().expect("no progam name?");
    let day: u8 = args
        .next()
        .context("pass a day via the first arg")?
        .parse()
        .context("day was not numeric")?;
    let day = Day::new(day);
    let mode: Mode = args
        .next()
        .expect("pass a mode via the second arg")
        .parse()?;

    match mode {
        Mode::Download => {
            let client = AOCClient::new(&config);
            client.load_prompt(day)?;
            client.load_input(day)?;
        }
        Mode::Sample => {
            run_solver_for_day(day, aoc_2024::solver::RunMode::Sample)?;
        }
        Mode::Input => {
            run_solver_for_day(day, aoc_2024::solver::RunMode::Input)?;
        }
    }

    Ok(())
}
