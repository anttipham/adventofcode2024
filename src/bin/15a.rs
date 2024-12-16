use std::{ error::Error, io::stdin };

use itertools::{ enumerate, Itertools };

fn main() -> Result<(), Box<dyn Error>> {
    let mut grid = stdin()
        .lines()
        .map(Result::unwrap)
        .take_while(|line| !line.is_empty())
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    let commands = stdin()
        .lines()
        .map(Result::unwrap)
        .map(|line| line.chars().collect_vec())
        .flatten();
    // println!("{:?}", grid);
    // println!("{}", commands);

    // Find the starting position
    let mut x = 0;
    let mut y = 0;
    for (i, row) in enumerate(&grid) {
        if let Some((j, _)) = row.iter().find_position(|&&c| c == '@') {
            grid[i][j] = '.';
            y = i as i32;
            x = j as i32;
            break;
        }
    }

    for c in commands {
        let (dir_x, dir_y) = match c {
            '<' => (-1, 0),
            '^' => (0, -1),
            '>' => (1, 0),
            'v' => (0, 1),
            _ => panic!("Invalid command: {}", c),
        };
        // Check for obstacles
        let mut i = 1;
        while grid[(y + i * dir_y) as usize][(x + i * dir_x) as usize] == 'O' {
            i += 1;
        }
        if grid[(y + i * dir_y) as usize][(x + i * dir_x) as usize] == '#' {
            continue;
        }
        grid[(y + i * dir_y) as usize][(x + i * dir_x) as usize] = 'O';
        grid[(y + dir_y) as usize][(x + dir_x) as usize] = '.';
        y += dir_y;
        x += dir_x;
    }

    let mut gps = 0;
    for (i, row) in enumerate(&grid) {
        for (j, &c) in enumerate(row) {
            if c == 'O' {
                gps += 100 * i + j;
            }
        }
    }
    println!("{}", gps);

    Ok(())
}
