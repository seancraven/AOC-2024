use anyhow::Result;

mod day1;
mod day2;
mod day3;
mod day4;
mod util;
#[tokio::main]
async fn main() -> Result<()> {
    // day1::day1().await?;
    // day2::main().await?;
    day4::main(true).await?;

    Ok(())
}
