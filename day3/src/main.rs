use anyhow::{anyhow, Result};
use regex::Regex;
use reqwest::{self, Client};

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::new();
    let resp = client
        .get("https://adventofcode.com/2024/day/3/input").header("Cookie", "_ga=GA1.2.1717327202.1733044324; _gid=GA1.2.1292154552.1733044324; _ga_MHSNPJKWC7=GS1.2.1733044324.1.1.1733044780.0.0.0; session=53616c7465645f5f67701f53ae986a3340db24598caa14dc7d6798a386463bc701a73c3aa8210a774b5df590752ee8d32a7ec12b1b18378586a16f4824d3d026")
        .send()
        .await?.text().await?;
    part_2(&resp)?;

    Ok(())
}
#[derive(Debug)]
enum Mode {
    Enabled,
    Disabled,
}
impl Mode {
    /// Change state after find the thingy.
    /// returns the last block value for the curren't mode.
    /// Find next also evolves the internal state, so
    /// If the mode starts enabled, it return's the final
    /// index that it is enabled for, and then set's it'self to disabled.
    fn find_next(&mut self, s: &str) -> Option<usize> {
        match self {
            Self::Enabled => {
                *self = Self::Disabled;
                s.find("don't()").map(|i| i + 7)
            }
            Self::Disabled => {
                *self = Self::Enabled;
                s.find("do()").map(|i| i + 4)
            }
        }
    }
    fn is_enabled(&self) -> bool {
        match &self {
            Self::Enabled => true,
            Self::Disabled => false,
        }
    }
}
fn part_2(s: &str) -> Result<()> {
    let mut mode = Mode::Enabled;
    let mut current_index = 0;
    let mut sum = 0;
    while let Some(mut next_idx) = mode.find_next(&s[current_index..]) {
        next_idx += current_index;
        let substring = &s[current_index..next_idx];
        if mode.is_enabled() {
            current_index = next_idx;
            continue;
        }
        parse_mul(substring, &mut sum)?;
        current_index = next_idx;
    }
    // if last block is a do block.
    if !mode.is_enabled() {
        parse_mul(&s[current_index..], &mut sum)?;
    }
    println!("Sum: {}", sum);
    Ok(())
}

fn parse_mul(s: &str, sum: &mut u64) -> Result<()> {
    let r = Regex::new(r"mul\([0-9]{1,3},[0-9]{1,3}\)")?;
    for m in r.find_iter(s) {
        let len = m.as_str().len();
        let (left, right) = m.as_str()[4..len - 1].split_once(',').unwrap();
        *sum += left.parse::<u64>().unwrap() * right.parse::<u64>().unwrap();
    }
    // println!("Q1: {}", sum);
    Ok(())
}
