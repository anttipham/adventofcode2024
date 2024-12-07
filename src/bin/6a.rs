use std::{ error::Error, io::stdin };

fn main() -> Result<(), Box<dyn Error>> {
    let mut x = 0;
    let mut y = 0;
    let mut map: Vec<Vec<char>> = {
        let mut i = 0;
        stdin()
            .lines()
            .map(Result::unwrap)
            .map(|line| {
                if let Some(j) = line.find('^') {
                    (y, x) = (i, j);
                }
                i += 1;
                let line = line.chars().collect();
                line
            })
            .collect()
    };

    let mut dx = 0;
    let mut dy = -1;
    loop {
        map[y][x] = 'X';
        // println!("({}, {})", x, y);

        let next_x = (x as i32) + dx;
        let next_y = (y as i32) + dy;
        if next_x < 0 || next_y < 0 || next_x >= (map.len() as i32) || next_y >= (map.len() as i32) {
            break;
        }
        if map[next_y as usize][next_x as usize] == '#' {
            let next_dx = -dy;
            let next_dy = dx;
            dx = next_dx;
            dy = next_dy;
            continue;
        }
        x = next_x as usize;
        y = next_y as usize;
    }
    let pos = map
        .iter()
        .flatten()
        .filter(|&&c| c == 'X')
        .count();
    println!("{}", pos);

    Ok(())
}
