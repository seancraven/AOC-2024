use std::collections::{BinaryHeap, HashMap};

use anyhow::{anyhow, Result};
use reqwest::{self, Client};

pub async fn day1() -> Result<()> {
    let client = Client::new();
    let resp = client
        .get("https://adventofcode.com/2024/day/1/input").header("Cookie", "_ga=GA1.2.1717327202.1733044324; _gid=GA1.2.1292154552.1733044324; _ga_MHSNPJKWC7=GS1.2.1733044324.1.1.1733044780.0.0.0; session=53616c7465645f5f67701f53ae986a3340db24598caa14dc7d6798a386463bc701a73c3aa8210a774b5df590752ee8d32a7ec12b1b18378586a16f4824d3d026")
        .send()
        .await?.text().await?;
    let mut heap_a = BinaryHeap::with_capacity(1000);
    let mut heap_b = BinaryHeap::with_capacity(1000);

    for line in resp.lines() {
        let (a, b) = line.split_once("   ").ok_or(anyhow!("Not found spaces"))?;
        heap_a.push(a.parse::<usize>()?);
        heap_b.push(b.parse::<usize>()?);
    }
    let mut sum = 0;
    let mut occurence = HashMap::with_capacity(1000);
    let mut a_s = Vec::with_capacity(heap_a.len());
    while let (Some(a), Some(b)) = (heap_a.pop(), heap_b.pop()) {
        sum += a.abs_diff(b);
        a_s.push(a);
        match occurence.get_mut(&b) {
            Some(occ) => {
                *occ += 1;
            }
            None => {
                occurence.insert(b, 1);
            }
        };
    }
    let mut sum_2 = 0;
    for a in a_s {
        sum_2 += a * occurence.get(&a).unwrap_or(&0);
    }
    println!("Solution {}", sum);

    println!("Question 2:");
    println!("Solution {}", sum_2);
    Ok(())
}
