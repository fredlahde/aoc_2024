use aoc_2024::{
    aoc_api::{AOCClient, Day},
    solver::run_solver_for_day,
};

fn main() -> anyhow::Result<()> {
    let config = aoc_2024::load_config()?;

    let client = AOCClient::new(&config);
    let day = Day::new(1);
    client.load_prompt(day)?;
    client.load_input(day)?;

    run_solver_for_day(day, aoc_2024::solver::RunMode::Input)?;
    Ok(())
}
