use std::{
    fmt::Display,
    fs::OpenOptions,
    io::{Read, Write},
    path::PathBuf,
    process::{Command, Stdio},
    str::FromStr,
    sync::Arc,
};

use reqwest::{
    blocking::{Client, ClientBuilder},
    cookie::Jar,
    Url,
};

use anyhow::{bail, Context, Result};

use crate::config::{Config, INPUTS_DIR, PROMPTS_DIR, SAMPLES_DIR};

const URL: &str = "adventofcode.com";
const END_OF_PUZZLE_PROMPT: &str = "Both parts of this puzzle are complete";

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct Day(u8);

impl Display for Day {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Day {
    pub fn new(day: u8) -> Self {
        if !(1..=25).contains(&day) {
            panic!("invalid day")
        }

        Self(day)
    }

    pub fn prompt_path(&self) -> PathBuf {
        PathBuf::from_str(&format!("{PROMPTS_DIR}/{:02}.md", self.0)).unwrap()
    }

    pub fn input_path(&self) -> PathBuf {
        PathBuf::from_str(&format!("{INPUTS_DIR}/{:02}.txt", self.0)).unwrap()
    }

    pub fn sample_path(&self) -> PathBuf {
        PathBuf::from_str(&format!("{SAMPLES_DIR}/{:02}.txt", self.0)).unwrap()
    }
}

pub struct AOCClient {
    client: Client,
}

impl AOCClient {
    pub fn new(config: &Config) -> Self {
        let jar = Jar::default();
        let cookie = format!("session={}; domain=.{URL}", config.session_cookie);
        let url = format!("https://.{URL}").parse::<Url>().unwrap();

        jar.add_cookie_str(&cookie, &url);

        let client = ClientBuilder::new()
            .cookie_store(true)
            .cookie_provider(Arc::new(jar))
            .build()
            .expect("able to build client");
        Self { client }
    }
}

impl AOCClient {
    pub fn load_prompt(&self, day: Day) -> Result<()> {
        let url = format!("https://{URL}/2024/day/{}", day.0);
        let resp = self
            .client
            .get(url)
            .send()
            .context("failed to load puzzle prompt")?;
        let text = resp.text().context("failed to read puzzle prompt")?;
        let mut cmd = Command::new("pandoc")
            .arg("--standalone")
            .arg("--from")
            .arg("html")
            .arg("--to")
            .arg("markdown")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .context("failed to spawn pandoc to convert from html to markdown")?;

        let mut stdin = cmd
            .stdin
            .take()
            .context("failed to claim stdin of pandoc")?;

        stdin
            .write_all(text.as_bytes())
            .context("failed to write html to pandoc")?;

        drop(stdin);

        let mut markdown = String::new();
        let mut stdout = cmd
            .stdout
            .take()
            .context("failed to claim stdout of pandoc")?;

        stdout.read_to_string(&mut markdown)?;

        let status = cmd.wait().context("failed to wait for pandoc to finish")?;

        if !status.success() {
            bail!("pandoc failed to convert html to markdown");
        }

        let lines = markdown.split('\n');
        let mut relevant_lines = Vec::new();
        for line in lines {
            relevant_lines.push(line);
            if line.contains(END_OF_PUZZLE_PROMPT) {
                break;
            }
        }

        let relevant_markdown = relevant_lines.join("\n");

        let out_path = format!("{PROMPTS_DIR}/{:02}.md", day.0);
        let mut out_fd = OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(out_path)
            .context("failed to open file for prompt")?;

        out_fd
            .write_all(relevant_markdown.as_bytes())
            .context("failed to write prompt to file")?;
        Ok(())
    }

    pub fn load_input(&self, day: Day) -> Result<()> {
        let url = format!("https://{URL}/2024/day/{}/input", day.0);
        let resp = self
            .client
            .get(url)
            .send()
            .context("failed to load puzzle prompt")?;
        let text = resp.text().context("failed to read puzzle prompt")?;

        let out_path = format!("{INPUTS_DIR}/{:02}.txt", day.0);
        let mut out_fd = OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(out_path)
            .context("failed to open file for prompt")?;

        out_fd
            .write_all(text.as_bytes())
            .context("failed to write prompt to file")?;
        Ok(())
    }
}
