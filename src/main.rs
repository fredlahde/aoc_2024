use aoc_2024::aoc_api::{AOCClient, Day};

fn main() -> anyhow::Result<()> {
    let config = aoc_2024::load_config()?;

    let client = AOCClient::new(&config);
    let day = Day::new(1);
    client.load_prompt(day)?;
    client.load_input(day)?;
    Ok(())
}