use std::{ error::Error, io::stdin };

fn main() -> Result<(), Box<dyn Error>> {
    let mut stones: Vec<u64> = stdin()
        .lines()
        .map(Result::unwrap)
        .flat_map(|input| input.split_whitespace().map(str::to_string).collect::<Vec<String>>())
        .map(|s| s.parse::<u64>().unwrap())
        .collect();

    for _ in 0..25 {
        let mut new_stones = Vec::new();
        for stone in stones {
            if stone == 0 {
                new_stones.push(1);
                continue;
            }
            let stone_length = stone.to_string().len() as u32;
            if stone_length % 2 == 0 {
                let divisor = (10u64).pow(stone_length / 2);
                let left_half = stone / divisor;
                let right_half = stone % divisor;
                new_stones.push(left_half);
                new_stones.push(right_half);
                continue;
            }
            new_stones.push(2024 * stone);
        }
        stones = new_stones;
    }

    println!("{}", stones.len());

    Ok(())
}
