use std::{ error::Error, io::stdin };

use itertools::Itertools;

fn main() -> Result<(), Box<dyn Error>> {
    let grid_size = (101i128, 103i128);
    // let grid_size = (11i128, 7i128);
    let seconds = 100i128;

    let reg = regex::Regex::new(r"(-?\d+),(-?\d+)")?;

    let mut quadrants = vec![0; 4];
    for line in stdin().lines().map(Result::unwrap) {
        let (p, v) = reg
            .captures_iter(&line)
            .map(|capture| capture.extract())
            .map(|(_, coords)| coords.map(|s| s.parse::<i128>().unwrap()))
            .map(|[x1, x2]| (x1, x2))
            .collect_tuple()
            .unwrap();

        let next_x = (p.0 + v.0 * seconds).rem_euclid(grid_size.0);
        let next_y = (p.1 + v.1 * seconds).rem_euclid(grid_size.1);

        let mid_quadrant_x = grid_size.0 / 2;
        let mid_quadrant_y = grid_size.1 / 2;
        if next_x == mid_quadrant_x || next_y == mid_quadrant_y {
            continue;
        }

        let left_quadrant = next_x < mid_quadrant_x;
        let top_quadrant = next_y < mid_quadrant_y;
        quadrants[(top_quadrant as usize) * 2 + (left_quadrant as usize)] += 1;
    }
    println!("{}", quadrants.iter().product::<i32>());

    Ok(())
}
