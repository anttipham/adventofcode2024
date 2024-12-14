use std::{ collections::HashSet, error::Error, io::stdin };

use itertools::Itertools;

fn main() -> Result<(), Box<dyn Error>> {
    let grid_size = (101i128, 103i128);
    // let grid_size = (11i128, 7i128);

    let reg = regex::Regex::new(r"(-?\d+),(-?\d+)")?;

    let mut robots = vec![];
    for line in stdin().lines().map(Result::unwrap) {
        let (p, v) = reg
            .captures_iter(&line)
            .map(|capture| capture.extract())
            .map(|(_, coords)| coords.map(|s| s.parse::<i128>().unwrap()))
            .map(|[x1, x2]| (x1, x2))
            .collect_tuple()
            .unwrap();
        robots.push((p, v));
    }

    for i in 0..100 {
        let seconds = 68 + i * 101;
        let loc = robots
            .iter()
            .map(|&(p, v)| {
                let next_x = (p.0 + v.0 * seconds).rem_euclid(grid_size.0);
                let next_y = (p.1 + v.1 * seconds).rem_euclid(grid_size.1);
                (next_x, next_y)
            })
            .collect::<HashSet<(i128, i128)>>();

        println!("Seconds: {}", seconds);
        for y in 0..grid_size.1 {
            for x in 0..grid_size.0 {
                if loc.contains(&(x, y)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
        println!();
    }

    Ok(())
}
