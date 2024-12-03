use std::{ collections::HashMap, io };

fn main() {
    let mut a: Vec<i32> = vec![];
    let mut b = HashMap::new();
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
        *b.entry(nums[1]).or_default() += 1;
    }
    a.sort();

    let mut sum = 0;
    for x in a {
        sum += x * b.get(&x).unwrap_or(&0);
    }
    println!("{sum}");
}
