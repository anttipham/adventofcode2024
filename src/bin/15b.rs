use std::{ error::Error, io::stdin };

use itertools::{ enumerate, Itertools };

fn main() -> Result<(), Box<dyn Error>> {
    let mut grid = stdin()
        .lines()
        .map(Result::unwrap)
        .take_while(|line| !line.is_empty())
        .map(|line|
            line
                .chars()
                .flat_map(|c| {
                    match c {
                        '#' => ['#', '#'],
                        'O' => ['[', ']'],
                        '.' => ['.', '.'],
                        '@' => ['@', '.'],
                        _ => panic!("Invalid character: {}", c),
                    }
                })
                .collect_vec()
        )
        .collect_vec();
    let commands = stdin()
        .lines()
        .map(Result::unwrap)
        .map(|line| line.chars().collect_vec())
        .flatten();

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
        if !can_move_box(&grid, (x + dir_x, y + dir_y), (dir_x, dir_y)) {
            continue;
        }
        move_box(&mut grid, (x + dir_x, y + dir_y), (dir_x, dir_y));
        y += dir_y;
        x += dir_x;

        // grid.iter().for_each(|row| {
        //     println!("{}", row.iter().collect::<String>());
        // });
    }

    let mut gps = 0;
    for (i, row) in enumerate(&grid) {
        for (j, &c) in enumerate(row) {
            if c == '[' {
                gps += 100 * i + j;
            }
        }
    }
    println!("{}", gps);

    Ok(())
}

fn can_move_box(grid: &Vec<Vec<char>>, (x, y): (i32, i32), (dir_x, dir_y): (i32, i32)) -> bool {
    let (left, right) = match grid[y as usize][x as usize] {
        '[' => ((x, y), (x + 1, y)),
        ']' => ((x - 1, y), (x, y)),
        '.' => {
            return true;
        }
        _ => {
            return false;
        }
    };
    if dir_y != 0 {
        return can_move_box(grid, (left.0, left.1 + dir_y), (dir_x, dir_y)) &&
            can_move_box(grid, (right.0, right.1 + dir_y), (dir_x, dir_y));
    } else if dir_x == 1 {
        return can_move_box(grid, (right.0 + 1, right.1), (dir_x, dir_y));
    } else if dir_x == -1 {
        return can_move_box(grid, (left.0 - 1, left.1), (dir_x, dir_y));
    }
    panic!("Invalid direction");
}

fn move_box(grid: &mut Vec<Vec<char>>, (x, y): (i32, i32), (dir_x, dir_y): (i32, i32)) {
    let (left, right) = match grid[y as usize][x as usize] {
        '[' => ((x, y), (x + 1, y)),
        ']' => ((x - 1, y), (x, y)),
        _ => {
            return;
        }
    };
    if dir_y != 0 {
        move_box(grid, (left.0, left.1 + dir_y), (dir_x, dir_y));
        move_box(grid, (right.0, right.1 + dir_y), (dir_x, dir_y));
    } else if dir_x == 1 {
        move_box(grid, (right.0 + 1, right.1), (dir_x, dir_y));
    } else if dir_x == -1 {
        move_box(grid, (left.0 - 1, left.1), (dir_x, dir_y));
    }
    grid[left.1 as usize][left.0 as usize] = '.';
    grid[right.1 as usize][right.0 as usize] = '.';
    grid[(left.1 + dir_y) as usize][(left.0 + dir_x) as usize] = '[';
    grid[(right.1 + dir_y) as usize][(right.0 + dir_x) as usize] = ']';
}
