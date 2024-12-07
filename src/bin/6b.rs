use std::{ collections::HashSet, error::Error, io::stdin };

#[derive(Debug, PartialEq, Eq, Hash)]
struct GuardStatus {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
}

fn guard_infinite_loop(map: &Vec<Vec<char>>, x: i32, y: i32) -> bool {
    let mut guard_path = HashSet::new();
    let mut x = x;
    let mut y = y;
    let mut dx = 0;
    let mut dy = -1;
    loop {
        if !guard_path.insert(GuardStatus { x, y, dx, dy }) {
            return true;
        }
        // println!("({}, {})", x, y);

        let next_x = (x as i32) + dx;
        let next_y = (y as i32) + dy;
        if next_x < 0 || next_y < 0 || next_x >= (map.len() as i32) || next_y >= (map.len() as i32) {
            return false;
        }
        if map[next_y as usize][next_x as usize] == '#' {
            let next_dx = -dy;
            let next_dy = dx;
            dx = next_dx;
            dy = next_dy;
            continue;
        }
        x = next_x;
        y = next_y;
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut x = 0;
    let mut y = 0;
    let map: Vec<Vec<char>> = {
        let mut i = 0;
        stdin()
            .lines()
            .map(Result::unwrap)
            .map(|line| {
                if let Some(j) = line.find('^') {
                    (y, x) = (i, j as i32);
                }
                i += 1;
                let line = line.chars().collect();
                line
            })
            .collect()
    };

    let mut sum = 0;
    // Can be optimized by iterating only on spaces where the guard visits
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] != '.' {
                continue;
            }
            let mut new_map = map.clone();
            new_map[i][j] = '#';
            if guard_infinite_loop(&new_map, x, y) {
                sum += 1;
            }
        }
    }
    println!("{}", sum);

    Ok(())
}
