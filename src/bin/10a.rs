use std::{ collections::HashSet, error::Error, io::stdin };

fn find_9(grid: &Vec<Vec<u32>>, x: i32, y: i32) -> HashSet<(i32, i32)> {
    if grid[y as usize][x as usize] == 9 {
        return HashSet::from([(x, y)]);
    }

    let mut goals = HashSet::new();
    for (next_x, next_y) in [
        (x - 1, y),
        (x + 1, y),
        (x, y - 1),
        (x, y + 1),
    ] {
        if
            next_x < 0 ||
            next_y < 0 ||
            next_x >= (grid[0].len() as i32) ||
            next_y >= (grid.len() as i32)
        {
            continue;
        }
        if grid[y as usize][x as usize] + 1 != grid[next_y as usize][next_x as usize] {
            continue;
        }
        goals.extend(find_9(grid, next_x, next_y));
    }
    goals
}

fn main() -> Result<(), Box<dyn Error>> {
    let grid: Vec<Vec<u32>> = stdin()
        .lines()
        .map(Result::unwrap)
        .map(|line|
            line
                .chars()
                .map(|c| c.to_digit(10).unwrap_or(99))
                .collect()
        )
        .collect();

    let mut sum = 0;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == 0 {
                let a = find_9(&grid, x as i32, y as i32).len();
                // println!("{}", a);
                sum += a;
            }
        }
    }

    println!("{}", sum);

    Ok(())
}
