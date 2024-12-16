use std::{ error::Error, io::stdin };

use itertools::Itertools;

fn main() -> Result<(), Box<dyn Error>> {
    let grid = stdin()
        .lines()
        .map(Result::unwrap)
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let mut queue = std::collections::binary_heap::BinaryHeap::new();
    queue.push((0i32, (1i32, (grid.len() as i32) - 2), (1i32, 0i32)));
    let mut visited = std::collections::HashSet::new();
    let points = loop {
        let (points, (x, y), (dx, dy)) = queue.pop().unwrap();
        if x == (grid[0].len() as i32) - 2 && y == 1 {
            break points;
        }
        if !visited.insert(((x, y), (dx, dy))) {
            continue;
        }
        // Move forward
        if grid[(y + dy) as usize][(x + dx) as usize] != '#' {
            queue.push((points - 1, (x + dx, y + dy), (dx, dy)));
        }
        // Turn left
        queue.push((points - 1000, (x, y), (-dy, dx)));
        // Turn right
        queue.push((points - 1000, (x, y), (dy, -dx)));
    };
    println!("{}", -points);

    Ok(())
}
