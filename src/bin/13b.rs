use std::{ error::Error, io::{ stdin, Read }, i64 };

use itertools::Itertools;

struct Machine {
    u: (i64, i64),
    v: (i64, i64),
    p: (i64, i64),
}

impl Machine {
    fn new(v: Vec<i64>) -> Self {
        Self {
            u: (v[0], v[1]),
            v: (v[2], v[3]),
            p: (10000000000000 + v[4], 10000000000000 + v[5]),
        }
    }
}

fn solve_machine(m: Machine) -> i64 {
    let y_numerator = m.p.1 * m.u.0 - m.p.0 * m.u.1;
    let y_denominator = m.v.1 * m.u.0 - m.v.0 * m.u.1;
    if y_denominator == 0 || y_numerator % y_denominator != 0 {
        return i64::MAX;
    }
    let y = y_numerator / y_denominator;
    let x_numerator = m.p.0 - y * m.v.0;
    let x_denominator = m.u.0;
    if x_denominator == 0 || x_numerator % x_denominator != 0 {
        return i64::MAX;
    }
    let x = x_numerator / x_denominator;
    if y < 0 || x < 0 {
        return i64::MAX;
    }
    3 * x + y
}

fn main() -> Result<(), Box<dyn Error>> {
    let regex = regex::Regex::new(r"X.?(\d+), Y.?(\d+)")?;
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;
    let sum: i64 = input
        .trim()
        .split("\n\n")
        .map(|machine|
            regex
                .captures_iter(machine)
                .flat_map(|cap|
                    cap
                        .iter()
                        .skip(1)
                        .map(|x| x.unwrap().as_str().parse::<i64>().unwrap())
                        .collect_vec()
                )
                .collect_vec()
        )
        // .inspect(|v| println!("{:?}", v))
        .map(Machine::new)
        .map(solve_machine)
        .filter(|&token| token != i64::MAX)
        // .inspect(|x| println!("{}", x))
        .sum();
    println!("{}", sum);

    Ok(())
}
