use std::{error::Error, io};

fn main() -> Result<(), Box<dyn Error>> {
    for line in io::stdin().lines() {
        let line = line.unwrap();
        if line.trim().is_empty() {
            break;
        }
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
    }

    Ok(())
}
