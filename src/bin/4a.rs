use std::{ error::Error, io };

type Grid = Vec<Vec<char>>;

fn rotate(grid: &Grid) -> Grid {
    let mut new_grid: Grid = vec![vec!['.'; grid.len()]; grid.len()];
    let n = grid.len() - 1;
    for i in 0..grid.len() {
        for j in 0..grid.len() {
            new_grid[n - j][i] = grid[i][j];
        }
    }
    new_grid
}

fn count_xmas(grid: &Grid) -> i32 {
    let mut count = 0;
    for i in 0..grid.len() {
        for j in 0..grid.len() - 3 {
            let text = &grid[i][j..j + 4];
            if text == ['X', 'M', 'A', 'S'] {
                count += 1;
            }
        }
    }
    for i in 0..grid.len() - 3 {
        for j in 0..grid.len() - 3 {
            let text = [grid[i][j], grid[i + 1][j + 1], grid[i + 2][j + 2], grid[i + 3][j + 3]];
            if text == ['X', 'M', 'A', 'S'] {
                count += 1;
            }
        }
    }
    count
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut grid: Grid = Vec::new();

    for line in io::stdin().lines() {
        let line = line?;
        let line = line.trim();
        if line.is_empty() {
            break;
        }
        grid.push(line.chars().collect());
    }

    let mut count = 0;
    for _ in 0..4 {
        count += count_xmas(&grid);
        grid = rotate(&grid);
    }
    println!("{}", count);

    Ok(())
}
