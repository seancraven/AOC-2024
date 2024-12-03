use std::{fs::File, io::Read};

use anyhow::Result;
use reqwest::Client;

#[tokio::main]
async fn main() -> Result<()> {
    // let url = "https://adventofcode.com/2024/day/2/input";
    // let client = Client::new();
    // let resp = client
    //     .get(url).header("Cookie", "_ga=GA1.2.1717327202.1733044324; _gid=GA1.2.1292154552.1733044324; _ga_MHSNPJKWC7=GS1.2.1733044324.1.1.1733044780.0.0.0; session=53616c7465645f5f67701f53ae986a3340db24598caa14dc7d6798a386463bc701a73c3aa8210a774b5df590752ee8d32a7ec12b1b18378586a16f4824d3d026")
    //     .send()
    //     .await?.text().await?;
    //
    let mut resp = String::new();
    File::open("input.txt")?.read_to_string(&mut resp)?;

    let mut out = 0;
    let mut rows = Vec::new();
    let mut fails = Vec::new();
    for line in resp.lines() {
        let current_iter = line.split(' ').map(|i| i.parse::<i32>().unwrap());
        let row = current_iter.clone().collect::<Vec<i32>>();
        rows.push(row.clone());
        let row_val = evaluate_row(current_iter);
        out += row_val;
        if row_val == 0 {
            fails.push(row);
        }
    }
    println!("Q1: {}", out);

    let mut q2 = 0;
    let mut new_rows = Vec::new();
    for line in &rows {
        let mut iter = line.iter().cloned();
        let mut skipped = false;
        let mut pos = None;
        let mut row = Vec::new();
        let mut prev = iter.next().unwrap();
        row.push(prev);
        for item in iter {
            let d = item - prev;
            if !pass(d, pos) && !skipped {
                skipped = true;
                continue;
            }
            prev = item;
            pos = Some(d > 0);
            row.push(item);
        }
        new_rows.push(row);
    }
    //
    //
    let mut o2 = 0;
    for row in rows.clone() {
        let o = evaluate_row(row.iter().cloned());
        if o == 1 {
            o2 += 1;
            continue;
        }
        for i in 0..row.len() {
            let mut new_row = row.clone();
            new_row.remove(i);
            let o = evaluate_row(new_row.into_iter());
            if o == 1 {
                o2 += 1;
                break;
            }
        }
    }
    println!("q2:{}", o2);
    for (row, old_row) in new_rows.iter().zip(&rows) {
        // assert_eq!(row, old_row);
        q2 += evaluate_row(row.iter().cloned());
    }
    println!("Q2: {}", out);
    Ok(())
}

fn evaluate_row(mut iter: impl Iterator<Item = i32>) -> i32 {
    let mut positive = None;
    let mut pass_: bool = false;
    let mut prev = iter.next().unwrap();
    for item in iter {
        let d = item - prev;
        pass_ = pass(d, positive);
        if !pass_ {
            break;
        }
        positive = Some(d > 0);
        prev = item;
        assert!(d != 0);
        assert!(d.abs() < 4, "d.abs {}", d.abs());
        assert!(d.abs() > 0);
    }
    pass_ as i32
}

fn pass(delta: i32, positive: Option<bool>) -> bool {
    if delta == 0 {
        return false;
    }
    let direction = match (positive, delta > 0) {
        (None, _) => true,
        (Some(p), d) => p == d,
    };
    delta.abs() < 4 && delta.abs() > 0 && direction
}
