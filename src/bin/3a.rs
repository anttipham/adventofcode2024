fn main() {
    let re = regex::Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut input = String::new();
    std::io
        ::stdin()
        .lines()
        .map(|line| line.unwrap().trim().to_string())
        .filter(|line| !line.is_empty())
        .for_each(|line| {
            input += &line;
        });

    let sum: i32 = re
        .captures_iter(&input)
        .map(|captures|
            captures
                .iter()
                .skip(1)
                .map(|capture| {
                    let num = capture.unwrap().as_str();
                    num.parse::<i32>().unwrap()
                })
                .product::<i32>()
        )
        .sum();
    println!("{}", sum);
}
