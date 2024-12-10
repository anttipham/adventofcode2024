use std::{ error::Error, io::stdin };
// use itertools::Itertools;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Block {
    size: usize,
    index: usize,
}

const N: usize = 1 << 15;

fn main() -> Result<(), Box<dyn Error>> {
    let mut file_blocks: Vec<i32> = Vec::new();
    let mut empty_block_tree: Vec<Block> = vec![Block { size: 0, index: 0 }; N]; // size -> indices
    let mut filled_blocks: Vec<Block> = Vec::new(); // (index, size)
    let mut id = 0;
    let mut tree_index = N / 2;
    stdin()
        .lines()
        .map(Result::unwrap)
        .next()
        .unwrap()
        .chars()
        .enumerate()
        .for_each(|(i, size)| {
            let size = size.to_digit(10).unwrap() as usize;
            let block = Block { index: file_blocks.len(), size };
            if i % 2 == 0 {
                filled_blocks.push(block);
                file_blocks.extend(vec![id; size]);
                id += 1;
            } else {
                empty_block_tree[tree_index] = block;
                tree_index += 1;
                file_blocks.extend(vec![-1; size]);
            }
        });

    for i in (1..N / 2).rev() {
        let left = &empty_block_tree[2 * i];
        let right = &empty_block_tree[2 * i + 1];
        empty_block_tree[i] = Block {
            index: 0,
            size: left.size.max(right.size),
        };
    }

    // println!(
    //     "{}",
    //     file_blocks
    //         .iter()
    //         .map(i32::to_string)
    //         .map(|x| if x == "-1" { ".".to_string() } else { x })
    //         .join("")
    // );
    for block in filled_blocks.iter().rev() {
        let mut i = 1;
        while i < N / 2 {
            let left = &empty_block_tree[2 * i];
            let right = &empty_block_tree[2 * i + 1];
            if block.size <= left.size {
                i = 2 * i;
            } else if block.size <= right.size {
                i = 2 * i + 1;
            } else {
                break;
            }
        }
        if i < N / 2 {
            continue;
        }
        let empty_block = empty_block_tree.get_mut(i).unwrap();
        if empty_block.index >= block.index {
            continue;
        }

        for i in 0..block.size {
            file_blocks.swap(block.index + i, empty_block.index + i);
        }
        // Fix data structure
        empty_block.size -= block.size;
        empty_block.index += block.size;
        while i > 0 {
            i /= 2;
            let left = &empty_block_tree[2 * i];
            let right = &empty_block_tree[2 * i + 1];
            empty_block_tree[i] = Block {
                index: 0,
                size: left.size.max(right.size),
            };
        }

        // println!(
        //     "{}",
        //     file_blocks
        //         .iter()
        //         .map(i32::to_string)
        //         .map(|x| if x == "-1" { ".".to_string() } else { x })
        //         .join("")
        // );
    }

    let sum: usize = file_blocks
        .iter()
        .map(|&x| if x == -1 { 0 } else { x })
        .enumerate()
        .map(|(i, x)| i * (x as usize))
        .sum();

    println!("{}", sum);

    Ok(())
}
