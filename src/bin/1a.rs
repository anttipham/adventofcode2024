use std::{io, iter::zip};

fn main() {
    let mut a: Vec<i32> = vec![];
    let mut b: Vec<i32> = vec![];
    for line in io::stdin().lines() {
        let line = line.unwrap();
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .take(2)
            .collect();
        if nums.len() != 2 {
            break;
        }
        a.push(nums[0]);
        b.push(nums[1]);
    }
    a.sort();
    b.sort();

    let mut sum = 0;
    for (x, y) in zip(a, b) {
        sum += (x - y).abs();
    }
    println!("{sum}");
}
