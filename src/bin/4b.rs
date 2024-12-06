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
    for i in 0..grid.len() - 2 {
        for j in 0..grid.len() - 2 {
            let m1 = grid[i][j];
            let m2 = grid[i + 2][j];
            let a = grid[i + 1][j + 1];
            let s1 = grid[i][j + 2];
            let s2 = grid[i + 2][j + 2];
            if m1 == 'M' && m2 == 'M' && a == 'A' && s1 == 'S' && s2 == 'S' {
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
