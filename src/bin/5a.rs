use std::{ collections::HashMap, error::Error, io };

fn main() -> Result<(), Box<dyn Error>> {
    let mut prev_pages = HashMap::new();
    let mut next_pages = HashMap::new();
    let mut page_updates = vec![];

    for line in io::stdin().lines() {
        let line = line.unwrap();
        if line.is_empty() {
            break;
        }

        let nums: Vec<i32> = line
            .split("|")
            .filter_map(|x| x.parse().ok())
            .collect();

        next_pages
            .entry(nums[0])
            .or_insert(vec![])
            .push(nums[1]);
        prev_pages
            .entry(nums[1])
            .or_insert(vec![])
            .push(nums[0]);
    }
    for line in io::stdin().lines() {
        let line = line.unwrap();
        if line.is_empty() {
            break;
        }
        let page_update: Vec<i32> = line
            .split(",")
            .map(|x| x.parse().unwrap())
            .collect();
        page_updates.push(page_update);
    }

    let mut sum = 0;
    for page_update in &page_updates {
        let mut correct_update = true;
        for i in 0..page_update.len() {
            let current = &page_update[i];
            let prev_order = match prev_pages.get(current) {
                Some(prev) => prev,
                None => {
                    continue;
                }
            };
            let next_order = match next_pages.get(current) {
                Some(next) => next,
                None => {
                    continue;
                }
            };
            let prev_pages = &page_update[..i].iter().all(|page| prev_order.contains(page));
            let next_pages = &page_update[i + 1..].iter().all(|page| next_order.contains(page));
            if !prev_pages || !next_pages {
                correct_update = false;
                break;
            }
        }
        if correct_update {
            sum += page_update[page_update.len() / 2];
        }
    }

    println!("{}", sum);

    Ok(())
}
