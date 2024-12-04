use std::isize;

use anyhow::{anyhow, Result};
use reqwest::Client;
const XMAS_BYTES: [u8; 4] = [88, 77, 65, 83];
const MAS_BYTES: [u8; 3] = [77, 65, 83];
pub async fn main(p1: bool) -> Result<()> {
    let client = Client::new();
    let resp = client
        .get("https://adventofcode.com/2024/day/4/input").header("Cookie", "_ga=GA1.2.1717327202.1733044324; _gid=GA1.2.1292154552.1733044324; _ga_MHSNPJKWC7=GS1.2.1733044324.1.1.1733044780.0.0.0; session=53616c7465645f5f67701f53ae986a3340db24598caa14dc7d6798a386463bc701a73c3aa8210a774b5df590752ee8d32a7ec12b1b18378586a16f4824d3d026")
        .send()
        .await?.text().await?;
    let mut sum = 0;
    let val = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
    if p1 {
        // let txt = ArrayOfText::try_from_str(val, XMAS_BYTES)?;
        let txt = ArrayOfText::try_from(resp, XMAS_BYTES)?;
        for j in 0..txt.rows {
            for i in 0..txt.line_length {
                sum += txt.left((i, j)) as usize;
                sum += txt.right((i, j)) as usize;
                sum += txt.up((i, j)) as usize;
                sum += txt.down((i, j)) as usize;
                sum += txt.up_diag_left((i, j)) as usize;
                sum += txt.up_diag_right((i, j)) as usize;
                sum += txt.down_diag_right((i, j)) as usize;
                sum += txt.down_diag_left((i, j)) as usize;
            }
        }
    } else {
        // let txt = ArrayOfText::try_from_str(val, MAS_BYTES)?;
        let txt = ArrayOfText::try_from(resp, MAS_BYTES)?;
        for j in 1..txt.rows - 1 {
            for i in 1..txt.line_length - 1 {
                if p1 {
                } else {
                    sum += ((txt.down_diag_right((i - 1, j - 1))
                        || txt.up_diag_left((i + 1, j + 1)))
                        && (txt.up_diag_right((i - 1, j + 1))
                            || txt.down_diag_left((i + 1, j - 1))))
                        as usize;
                }
            }
        }
    }
    println!("Sum: {}", sum);
    Ok(())
}

struct ArrayOfText<const N: usize> {
    body: String,
    line_length: usize,
    rows: usize,
    bytes: [u8; N],
}
impl<const N: usize> ArrayOfText<N> {
    fn at(&self, i: usize, j: usize) -> Option<u8> {
        if i >= self.line_length {
            return None;
        }
        if j >= self.rows {
            return None;
        }
        // Include newline
        let idx = i + j * (self.line_length + 1);
        if idx > self.body.len() {
            return None;
        }
        let c = self.body.as_bytes()[idx];
        Some(c)
    }
    fn right(&self, start: (usize, usize)) -> bool {
        self.check_xmas(start, (0..4).map(|i| (i, 0)))
    }
    fn left(&self, start: (usize, usize)) -> bool {
        self.check_xmas(start, (0..4).map(|i| (-i, 0)))
    }
    fn down(&self, start: (usize, usize)) -> bool {
        self.check_xmas(start, (0..4).map(|i| (0, i)))
    }
    fn up(&self, start: (usize, usize)) -> bool {
        self.check_xmas(start, (0..4).map(|i| (0, -i)))
    }
    fn down_diag_right(&self, start: (usize, usize)) -> bool {
        self.check_xmas(start, (0..4).map(|i| (i, i)))
    }
    fn down_diag_left(&self, start: (usize, usize)) -> bool {
        self.check_xmas(start, (0..4).map(|i| (-i, i)))
    }
    fn up_diag_left(&self, start: (usize, usize)) -> bool {
        self.check_xmas(start, (0..4).map(|i| (-i, -i)))
    }
    fn up_diag_right(&self, start: (usize, usize)) -> bool {
        self.check_xmas(start, (0..4).map(|i| (i, -i)))
    }
    fn check_xmas(
        &self,
        start: (usize, usize),
        iter: impl Iterator<Item = (isize, isize)>,
    ) -> bool {
        for (i, targ) in iter.zip(self.bytes) {
            let x = start.0 as isize + i.0;
            let y = start.1 as isize + i.1;
            if x < 0 || y < 0 {
                return false;
            }
            let Some(item) = self.at(x as usize, y as usize) else {
                return false;
            };
            if item != targ {
                return false;
            }
        }
        true
    }
    fn try_from(value: String, bytes: [u8; N]) -> Result<Self> {
        let line_lengths = value
            .split("\n")
            .filter(|row| !row.is_empty())
            .collect::<Vec<&str>>();
        let row_0 = line_lengths[0];
        for (i, row) in (line_lengths[1..]).iter().enumerate() {
            if row.len() != row_0.len() {
                return Err(anyhow!(
                    "Rows {}/{} aren't the same length must fail ",
                    *row,
                    line_lengths.len()
                ));
            }
        }
        let line_length = row_0.len();
        let rows = line_lengths.len();

        Ok(ArrayOfText {
            body: value,
            line_length,
            rows,
            bytes,
        })
    }
    fn try_from_str(value: &str, bytes: [u8; N]) -> Result<Self> {
        Self::try_from(String::from(value), bytes)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_right() {
        assert!(ArrayOfText::try_from_str("XMAS", XMAS_BYTES)
            .unwrap()
            .right((0, 0)));
    }
    #[test]
    fn test_left() {
        assert!(ArrayOfText::try_from_str("SAMX", XMAS_BYTES)
            .unwrap()
            .left((3, 0)));
    }
    #[test]
    fn test_down() {
        let input = "AAAX\nBBBM\nCCCA\nDDDS";
        assert!(ArrayOfText::try_from_str(input, XMAS_BYTES)
            .unwrap()
            .down((3, 0)));
    }
    #[test]
    fn test_up() {
        let input = "SAAX\nABBM\nMCCA\nXDDS";
        assert!(ArrayOfText::try_from_str(input, XMAS_BYTES)
            .unwrap()
            .up((0, 3)));
    }

    #[test]
    fn test_down_diag_right() {
        let input = "XAAA\nBMBB\nCCAC\nDDDS";
        assert!(ArrayOfText::try_from_str(input, XMAS_BYTES)
            .unwrap()
            .down_diag_right((0, 0)));
    }

    #[test]
    fn test_down_diag_left() {
        let input = "AAAX\nBBMB\nCACC\nSDDD";
        assert!(ArrayOfText::try_from_str(input, XMAS_BYTES)
            .unwrap()
            .down_diag_left((3, 0)));
    }

    #[test]
    fn test_up_diag_right() {
        let input = "AAAS\nBBAB\nCMCC\nXDDD";
        assert!(ArrayOfText::try_from_str(input, XMAS_BYTES)
            .unwrap()
            .up_diag_right((0, 3)));
    }

    #[test]
    fn test_up_diag_left() {
        let input = "SAAA\nBABB\nCCMC\nDDDX";
        assert!(ArrayOfText::try_from_str(input, XMAS_BYTES)
            .unwrap()
            .up_diag_left((3, 3)));
    }
}
