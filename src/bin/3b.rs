fn main() {
    let re = regex::Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();
    let mut input = String::new();
    std::io
        ::stdin()
        .lines()
        .map(|line| line.unwrap().trim().to_string())
        .filter(|line| !line.is_empty())
        .for_each(|line| {
            input += &line;
        });

    let mut sum = 0;
    let mut enabled = true;
    for captures in re.captures_iter(&input) {
        let full = captures.get(0).unwrap().as_str();
        if full == "do()" {
            enabled = true;
            continue;
        }
        if full == "don't()" {
            enabled = false;
            continue;
        }
        if !enabled {
            continue;
        }
        let nums = captures
            .iter()
            .skip(1)
            .map(|capture| capture.unwrap().as_str().parse::<i32>().unwrap());
        let product = nums.product::<i32>();
        sum += product;
    }
    println!("{sum}");
}
