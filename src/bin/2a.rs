use std::io;

fn is_safe(nums: Vec<i32>) -> bool {
    let df = nums[1] - nums[0];
    for i in 1..nums.len() {
        let diff = nums[i] - nums[i - 1];
        if diff == 0 || diff.abs() > 3 || diff * df < 0 {
            return false;
        }
    }
    true
}

fn main() {
    let mut sum = 0;
    for line in io::stdin().lines() {
        let line = line.unwrap();
        if line.trim().is_empty() {
            break;
        }
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        if is_safe(nums) {
            sum += 1;
        }
    }
    println!("{}", sum);
}
