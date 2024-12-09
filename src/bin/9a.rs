use std::{ error::Error, io::stdin };

fn main() -> Result<(), Box<dyn Error>> {
    let mut file_blocks: Vec<i32> = Vec::new();
    let mut id = 0;
    stdin()
        .lines()
        .map(Result::unwrap)
        .next()
        .unwrap()
        .chars()
        .enumerate()
        .for_each(|(i, num)| {
            let num = num.to_digit(10).unwrap() as usize;
            if i % 2 == 0 {
                file_blocks.extend(vec![id; num]);
                id += 1;
            } else {
                file_blocks.extend(vec![-1; num]);
            }
        });

    let mut i = 0;
    let mut j = file_blocks.len() - 1;
    while i < j {
        if file_blocks[i] != -1 {
            i += 1;
            continue;
        }
        if file_blocks[j] == -1 {
            j -= 1;
            continue;
        }
        file_blocks.swap(i, j);
        i += 1;
        j -= 1;
    }
    let sum: usize = file_blocks
        .iter()
        .filter(|&&x| x != -1)
        .enumerate()
        .map(|(i, &x)| i * x as usize)
        .sum();

    println!("{}", sum);

    Ok(())
}
