use anyhow::{Context, Result};
use reqwest::Client;
async fn get_text(day: usize) -> Result<String> {
    let client = Client::new();
    client
        .get(format!("https://adventofcode.com/2024/day/{}/input", day)).header("Cookie", "_ga=GA1.2.1717327202.1733044324; _gid=GA1.2.1292154552.1733044324; _ga_MHSNPJKWC7=GS1.2.1733044324.1.1.1733044780.0.0.0; session=53616c7465645f5f67701f53ae986a3340db24598caa14dc7d6798a386463bc701a73c3aa8210a774b5df590752ee8d32a7ec12b1b18378586a16f4824d3d026")
        .send()
        .await?.text().await.context("")
}
