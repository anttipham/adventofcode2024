use std::{ collections::HashSet, error::Error, io::stdin };

#[derive(Debug)]
struct Info {
    area: u32,
    sides: HashSet<((i32, i32), (i32, i32))>,
}

fn fill_info(
    grid: &Vec<Vec<char>>,
    visited: &mut HashSet<(i32, i32)>,
    x: i32,
    y: i32,
    plant: char,
    prev_x: i32,
    prev_y: i32
) -> Info {
    let mut info = Info { area: 0, sides: HashSet::new() };

    if x < 0 || y < 0 || x >= (grid[0].len() as i32) || y >= (grid.len() as i32) {
        info.sides.insert(((prev_x, prev_y), (x, y)));
        return info;
    }
    if plant != grid[y as usize][x as usize] {
        info.sides.insert(((prev_x, prev_y), (x, y)));
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
        let new_info = fill_info(grid, visited, nx, ny, plant, x, y);
        info.area += new_info.area;
        info.sides.extend(new_info.sides);
    }
    return info;
}

fn count_sides(sides: &mut HashSet<((i32, i32), (i32, i32))>) -> u32 {
    let mut count = 0;
    loop {
        // for (start, end) in sides
        let (start, end) = {
            let next = sides.iter().next();
            if next.is_none() {
                return count;
            }
            count += 1;
            next.unwrap().clone()
        };
        sides.remove(&(start, end));
        let diff_x = end.0 - start.0;
        let diff_y = end.1 - start.1;
        for dir in [-1, 1] {
            let dx = dir * diff_y;
            let dy = dir * diff_x;
            let mut next_start = start;
            let mut next_end = end;
            loop {
                next_start = (next_start.0 + dx, next_start.1 + dy);
                next_end = (next_end.0 + dx, next_end.1 + dy);
                if !sides.contains(&(next_start, next_end)) {
                    break;
                }
                sides.remove(&(next_start, next_end));
            }
        }
    }
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
            let info = fill_info(&grid, &mut visited, x as i32, y as i32, grid[y][x], 0, 0);
            let mut sides = info.sides.clone();
            // println!("{} {:?}", grid[y][x], sides.len());
            price += info.area * count_sides(&mut sides);
        }
    }
    println!("{}", price);

    Ok(())
}
