use std::io;

fn main() {
    for line in io::stdin().lines() {
        let line = line.unwrap();
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
    }
}
