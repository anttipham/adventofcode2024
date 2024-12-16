// use std::{ error::Error, io::stdin };

// use itertools::Itertools;

// struct Path {
//     x: i32,
//     y: i32,
//     dx: i32,
//     dy: i32,
//     points: i32,
//     visited: std::collections::HashSet<((i32, i32), (i32, i32))>,
//     route: Vec<(i32, i32)>,
// }

// impl Ord for Path {
//     fn cmp(&self, other: &Self) -> std::cmp::Ordering {
//         self.points.cmp(&other.points)
//     }
// }
// impl PartialOrd for Path {
//     fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
//         Some(self.cmp(other))
//     }
// }
// impl PartialEq for Path {
//     fn eq(&self, other: &Self) -> bool {
//         self.points == other.points
//     }
// }
// impl Eq for Path {}
// impl Path {
//     fn new(x: i32, y: i32) -> Self {
//         Self {
//             x,
//             y,
//             dx: 1,
//             dy: 0,
//             points: 0,
//             visited: std::collections::HashSet::new(),
//             route: Vec::new(),
//         }
//     }
// }

// fn main() -> Result<(), Box<dyn Error>> {
//     let grid = stdin()
//         .lines()
//         .map(Result::unwrap)
//         .map(|line| line.chars().collect_vec())
//         .collect_vec();

//     let mut queue = std::collections::binary_heap::BinaryHeap::new();
//     queue.push(Path::new(1i32, (grid.len() as i32) - 2));
//     let mut best_points = 0;
//     let mut best_routes = Vec::new();
//     loop {
//         let mut path = match queue.pop() {
//             Some(v) => v,
//             None => {
//                 break;
//             }
//         };
//         if path.x == (grid[0].len() as i32) - 2 && path.y == 1 {
//             if best_points == 0 {
//                 best_points = path.points;
//             }
//             if path.points < best_points {
//                 break;
//             }
//             best_routes.push(path.route);
//             continue;
//         }
//         if !path.visited.insert(((path.x, path.y), (path.dx, path.dy))) {
//             continue;
//         }
//         // Turn left
//         queue.push(Path {
//             points: path.points - 1000,
//             route: path.route.clone(),
//             dx: -path.dy,
//             dy: path.dx,
//             visited: path.visited.clone(),
//             ..path
//         });
//         // Turn right
//         queue.push(Path {
//             points: path.points - 1000,
//             route: path.route.clone(),
//             dx: path.dy,
//             dy: -path.dx,
//             visited: path.visited.clone(),
//             ..path
//         });
//         // Move forward
//         if grid[(path.y + path.dy) as usize][(path.x + path.dx) as usize] != '#' {
//             path.route.push((path.x, path.y));
//             queue.push(Path {
//                 points: path.points - 1,
//                 x: path.x + path.dx,
//                 y: path.y + path.dy,
//                 ..path
//             });
//         }
//     }
//     // println!("{:?}", best_points);
//     let tiles = best_routes.iter().flatten().unique().collect_vec();
//     // println!("{:?}", tiles);
//     println!("{:?}", tiles.iter().count() + 1);

//     // for i in 0..grid.len() {
//     //     for j in 0..grid[i].len() {
//     //         if grid[i][j] == '#' {
//     //             print!("#");
//     //         } else if tiles.contains(&&(j as i32, i as i32)) {
//     //             print!("O");
//     //         } else {
//     //             print!(".");
//     //         }
//     //     }
//     //     println!();
//     // }

//     Ok(())
// }
// use std::{ collections::HashSet, error::Error, io::stdin };

// use itertools::Itertools;

// fn main() -> Result<(), Box<dyn Error>> {
//     let grid = stdin()
//         .lines()
//         .map(Result::unwrap)
//         .map(|line| line.chars().collect_vec())
//         .collect_vec();

//     let mut queue = std::collections::binary_heap::BinaryHeap::new();
//     queue.push((0i32, (1i32, (grid.len() as i32) - 2), (1i32, 0i32)));
//     let mut visited = std::collections::HashSet::new();
//     let best_points = -(loop {
//         let (points, (x, y), (dx, dy)) = queue.pop().unwrap();
//         if x == (grid[0].len() as i32) - 2 && y == 1 {
//             break points;
//         }
//         if !visited.insert(((x, y), (dx, dy))) {
//             continue;
//         }
//         // Move forward
//         if grid[(y + dy) as usize][(x + dx) as usize] != '#' {
//             queue.push((points - 1, (x + dx, y + dy), (dx, dy)));
//         }
//         // Turn left
//         queue.push((points - 1000, (x, y), (-dy, dx)));
//         // Turn right
//         queue.push((points - 1000, (x, y), (dy, -dx)));
//     });
//     println!("{}", best_points);

//     // Find the paths
//     let mut all_visited_tiles = HashSet::new();
//     dfs(
//         &grid,
//         &mut all_visited_tiles,
//         &mut HashSet::new(),
//         (1i32, (grid.len() as i32) - 2),
//         (1, 0),
//         0,
//         best_points
//     );
//     println!("{}", all_visited_tiles.len() + 1);

//     Ok(())
// }
// fn dfs(
//     grid: &Vec<Vec<char>>,
//     all_visited_tiles: &mut HashSet<(i32, i32)>,
//     path: &mut HashSet<((i32, i32), (i32, i32))>,
//     cur: (i32, i32),
//     dir: (i32, i32),
//     points: i32,
//     best_points: i32
// ) {
//     // let (points, (x, y), (dx, dy)) = queue.pop().unwrap();
//     //     if x == (grid[0].len() as i32) - 2 && y == 1 {
//     //         break points;
//     //     }
//     //     if !visited.insert(((x, y), (dx, dy))) {
//     //         continue;
//     //     }
//     //     // Move forward
//     //     if grid[(y + dy) as usize][(x + dx) as usize] != '#' {
//     //         queue.push((points - 1, (x + dx, y + dy), (dx, dy)));
//     //     }
//     //     // Turn left
//     //     queue.push((points - 1000, (x, y), (-dy, dx)));
//     //     // Turn right
//     //     queue.push((points - 1000, (x, y), (dy, -dx)));
//     if points > best_points {
//         return;
//     }
//     if cur == ((grid[0].len() as i32) - 2, 1) {
//         all_visited_tiles.extend(path.iter().map(|x| x.0));
//         return;
//     }
//     if !path.insert((cur, dir)) {
//         return;
//     }
//     // Move forward
//     if grid[(cur.1 + dir.1) as usize][(cur.0 + dir.0) as usize] != '#' {
//         dfs(
//             grid,
//             all_visited_tiles,
//             path,
//             (cur.0 + dir.0, cur.1 + dir.1),
//             dir,
//             points + 1,
//             best_points
//         );
//     }
//     // Turn left
//     dfs(grid, all_visited_tiles, path, cur, (-dir.1, dir.0), points + 1000, best_points);
//     // Turn right
//     dfs(grid, all_visited_tiles, path, cur, (dir.1, -dir.0), points + 1000, best_points);
//     path.remove(&(cur, dir));
// }
// use std::{ collections::{BinaryHeap, HashMap, HashSet}, error::Error, io::stdin };

// use itertools::Itertools;

// fn main() -> Result<(), Box<dyn Error>> {
//     let grid = stdin()
//         .lines()
//         .map(Result::unwrap)
//         .map(|line| line.chars().collect_vec())
//         .collect_vec();

//     let mut best_points = i32::MAX;
//     let mut all_visited: HashMap<(i32, i32), i32> = HashMap::new();
//     let mut queue = BinaryHeap::new();
//     queue.push((0i32, (1i32, (grid.len() as i32) - 2), (1i32, 0i32)));
//     let mut visited = HashSet::new();
//     loop {
//         let (points, (x, y), (dx, dy)) = queue.pop().unwrap();
//         if points > best_points {
//             break;
//         }
//         if x == (grid[0].len() as i32) - 2 && y == 1 {
//             if best_points == i32::MAX {
//                 best_points = points;
//             }
//             all_visited.extend(visited.iter().map(|x: &((i32, i32), (i32, i32))| x.0));
//         }
//         if !visited.insert(((x, y), (dx, dy))) {
//             continue;
//         }
//         // Move forward
//         if grid[(y + dy) as usize][(x + dx) as usize] != '#' {
//             queue.push((points - 1, (x + dx, y + dy), (dx, dy)));
//         }
//         // Turn left
//         queue.push((points - 1000, (x, y), (-dy, dx)));
//         // Turn right
//         queue.push((points - 1000, (x, y), (dy, -dx)));
//     }
//     // println!("{}", -points);

//     Ok(())
// }
use std::{ collections::{ HashMap, HashSet }, error::Error, i32, io::stdin };

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
    let best_points = -(loop {
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
    });
    println!("{}", best_points);

    // Find the paths
    let mut routes = HashSet::new();
    dfs(
        &grid,
        &mut HashMap::new(),
        &mut routes,
        &mut HashSet::new(),
        (1i32, (grid.len() as i32) - 2),
        (1, 0),
        0,
        best_points
    );
    println!("{}", routes.len() + 1);

    Ok(())
}
fn dfs(
    grid: &Vec<Vec<char>>,
    grid_points: &mut HashMap<((i32, i32), (i32, i32)), i32>,
    routes: &mut HashSet<(i32, i32)>,
    path: &mut HashSet<(i32, i32)>,
    cur: (i32, i32),
    dir: (i32, i32),
    points: i32,
    best_points: i32
) {
    if points > best_points {
        return;
    }
    if cur == ((grid[0].len() as i32) - 2, 1) {
        routes.extend(path.iter());
        return;
    }
    let cur_grid_point = grid_points.entry((cur, dir)).or_insert(i32::MAX);
    if points > *cur_grid_point {
        return;
    }
    *cur_grid_point = points;
    if !path.insert(cur) {
        return;
    }
    [
        (1, 0),
        (0, 1),
        (-1, 0),
        (0, -1),
    ]
        .iter()
        .filter(|next_dir| (dir.0 != -next_dir.0 || dir.1 != -next_dir.1))
        .filter(|next_dir| {
            grid[(cur.1 + next_dir.1) as usize][(cur.0 + next_dir.0) as usize] != '#'
        })
        .for_each(|&next_dir| {
            let points = if dir == next_dir { points + 1 } else { points + 1001 };
            dfs(
                grid,
                grid_points,
                routes,
                path,
                (cur.0 + next_dir.0, cur.1 + next_dir.1),
                next_dir,
                points,
                best_points
            );
        });
    path.remove(&cur);
}
