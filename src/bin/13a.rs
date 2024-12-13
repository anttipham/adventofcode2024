use std::{ error::Error, io::{ stdin, Read }, u32 };

use itertools::Itertools;

struct Machine {
    ax: u32,
    ay: u32,
    bx: u32,
    by: u32,
    px: u32,
    py: u32,
}

impl Machine {
    fn new(v: Vec<u32>) -> Self {
        Self {
            ax: v[0],
            ay: v[1],
            bx: v[2],
            by: v[3],
            px: v[4],
            py: v[5],
        }
    }
}

fn solve_machine(machine: Machine) -> u32 {
    let mut cost = u32::MAX;
    for i in 0.. {
        if i * machine.ay > machine.py {
            return cost;
        }
        let j = (machine.py - i * machine.ay) / machine.by;
        let clawx = i * machine.ax + j * machine.bx;
        let clawy = i * machine.ay + j * machine.by;
        // if clawx > machine.px && clawy > machine.py {
        //     return cost;
        // }
        if clawx == machine.px && clawy == machine.py {
            cost = cost.min(3 * i + j);
        }
    }
    panic!("No solution found");
}

fn main() -> Result<(), Box<dyn Error>> {
    let regex = regex::Regex::new(r"X.?(\d+), Y.?(\d+)")?;
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;
    let sum: u32 = input
        .trim()
        .split("\n\n")
        .map(|machine|
            regex
                .captures_iter(machine)
                .flat_map(|cap|
                    cap
                        .iter()
                        .skip(1)
                        .map(|x| x.unwrap().as_str().parse::<u32>().unwrap())
                        .collect_vec()
                )
                .collect_vec()
        )
        // .inspect(|v| println!("{:?}", v))
        .map(Machine::new)
        .map(solve_machine)
        .filter(|&token| token != u32::MAX)
        // .inspect(|x| println!("{}", x))
        .sum();
    println!("{}", sum);

    Ok(())
}
