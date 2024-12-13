use std::{ collections::HashSet, error::Error, io::stdin };

#[derive(Debug)]
struct Info {
    area: u32,
    perimeter: u32,
}

fn fill_info(
    grid: &Vec<Vec<char>>,
    visited: &mut HashSet<(i32, i32)>,
    x: i32,
    y: i32,
    plant: char
) -> Info {
    let mut info = Info { area: 0, perimeter: 0 };

    if x < 0 || y < 0 || x >= (grid[0].len() as i32) || y >= (grid.len() as i32) {
        info.perimeter += 1;
        return info;
    }
    if plant != grid[y as usize][x as usize] {
        info.perimeter += 1;
        return info;
    }
    if visited.contains(&(x, y)) {
        return info;
    }
    visited.insert((x, y));
    info.area += 1;

    for (dx, dy) in &[
        (0, 1),
        (1, 0),
        (0, -1),
        (-1, 0),
    ] {
        let nx = x + dx;
        let ny = y + dy;
        let new_info = fill_info(grid, visited, nx, ny, plant);
        info.area += new_info.area;
        info.perimeter += new_info.perimeter;
    }
    return info;
}

fn main() -> Result<(), Box<dyn Error>> {
    let grid: Vec<Vec<char>> = stdin()
        .lines()
        .map(Result::unwrap)
        .map(|s| s.chars().collect())
        .collect();

    let mut visited = HashSet::new();

    let mut price = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            let info = fill_info(&grid, &mut visited, x as i32, y as i32, grid[y][x]);
            price += info.area * info.perimeter;
        }
    }
    println!("{}", price);

    Ok(())
}
