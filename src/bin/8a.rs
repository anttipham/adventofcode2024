use std::{ collections::{ HashMap, HashSet }, error::Error, io::stdin };

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
        for antenna1 in &coords {
            for antenna2 in &coords {
                if antenna1 == antenna2 {
                    continue;
                }
                let antinode = (
                    (2*antenna2.0 - antenna1.0),
                    (2*antenna2.1 - antenna1.1),
                );
                if 0 <= antinode.0 && antinode.0 < n && 0 <= antinode.1 && antinode.1 < n {
                    antinodes.insert(antinode);
                }
            }
        }
    }
    println!("{}", antinodes.len());

    Ok(())
}
