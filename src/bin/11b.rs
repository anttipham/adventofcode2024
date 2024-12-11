use std::{ collections::HashMap, error::Error, hash::Hash, io::stdin };

fn main() -> Result<(), Box<dyn Error>> {
    let mut stones: HashMap<u64, u64> = stdin()
        .lines()
        .map(Result::unwrap)
        .flat_map(|input| input.split_whitespace().map(str::to_string).collect::<Vec<String>>())
        .map(|s| s.parse::<u64>().unwrap())
        .map(|s| (s, 1))
        .collect();

    for _ in 0..75 {
        let mut new_stones = HashMap::new();
        for (stone, n) in stones {
            if stone == 0 {
                *new_stones.entry(1).or_default() += n;
                continue;
            }
            let stone_length = stone.to_string().len() as u32;
            if stone_length % 2 == 0 {
                let divisor = (10u64).pow(stone_length / 2);
                let left_half = stone / divisor;
                let right_half = stone % divisor;
                *new_stones.entry(left_half).or_default() += n;
                *new_stones.entry(right_half).or_default() += n;
                continue;
            }
            *new_stones.entry(2024 * stone).or_default() += n;
        }
        stones = new_stones;
    }

    let sum: u64 = stones.values().sum();
    println!("{sum}");

    Ok(())
}
