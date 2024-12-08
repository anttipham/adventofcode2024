use std::{ collections::{ HashMap, HashSet }, error::Error, io::stdin };

use itertools::Itertools;

fn main() -> Result<(), Box<dyn Error>> {
    let mut n = 0;
    let mut antennas = HashMap::new();
    for (i, line) in stdin().lines().map(Result::unwrap).enumerate() {
        if line.is_empty() {
            break;
        }
        if n == 0 {
            n = line.len() as i32;
        }

        line.match_indices(|c| c != '.').for_each(|(j, c)| {
            antennas
                .entry(c.to_string())
                .or_insert(vec![])
                .push((i as i32, j as i32));
        });
    }

    let mut antinodes = HashSet::new();
    for (_, coords) in antennas {
        for (antenna1, antenna2) in coords.iter().cartesian_product(coords.iter()) {
            if antenna1 == antenna2 {
                continue;
            }
            antinodes.extend(find_antinodes(n, antenna1, antenna2));
        }
    }
    println!("{}", antinodes.len());

    Ok(())
}

fn find_antinodes(n: i32, antenna1: &(i32, i32), antenna2: &(i32, i32)) -> HashSet<(i32, i32)> {
    let mut antinodes = HashSet::new();
    for i in 1.. {
        let antinode = (
            antenna1.0 + i * (antenna2.0 - antenna1.0),
            antenna1.1 + i * (antenna2.1 - antenna1.1),
        );
        if 0 <= antinode.0 && antinode.0 < n && 0 <= antinode.1 && antinode.1 < n {
            antinodes.insert(antinode);
        } else {
            break;
        }
    }
    antinodes
}
