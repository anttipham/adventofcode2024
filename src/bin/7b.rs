use std::{ error::Error, io::stdin };

use regex::Regex;

fn input() -> impl Iterator<Item = String> {
    stdin()
        .lines()
        .map(Result::unwrap)
        .map_while(|line| if line.is_empty() { None } else { Some(line) })
}

fn calc(res: i64, cur_res: i64, ops: &[i64]) -> bool {
    if cur_res > res {
        return false;
    }
    if ops.len() == 0 {
        if cur_res == res {
            return true;
        }
        return false;
    }
    calc(res, cur_res + ops[0], &ops[1..]) ||
        calc(res, cur_res * ops[0], &ops[1..]) ||
        calc(res, (cur_res.to_string() + &ops[0].to_string()).parse().unwrap(), &ops[1..])
}

fn main() -> Result<(), Box<dyn Error>> {
    let regex = Regex::new(r"(\d+): ([\d ]+)")?;
    let mut sum = 0;
    for line in input() {
        let caps = regex.captures(&line).unwrap();
        let mut iter = caps
            .iter()
            .map(|x| x.unwrap().as_str())
            .skip(1);
        let res: i64 = iter.next().unwrap().parse().unwrap();
        let eq: Vec<i64> = iter
            .next()
            .unwrap()
            .split_whitespace()
            .filter_map(|x| x.parse().ok())
            .collect();
        if calc(res, eq[0], &eq[1..]) {
            sum += res;
        }
    }

    println!("{}", sum);

    Ok(())
}
