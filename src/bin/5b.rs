use std::{ collections::HashMap, error::Error, io };

fn main() -> Result<(), Box<dyn Error>> {
    let mut prev_pages = HashMap::new();
    let mut next_pages = HashMap::new();

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

    let mut sum = 0;
    for line in io::stdin().lines() {
        let line = line.unwrap();
        if line.is_empty() {
            break;
        }
        let page_update: Vec<i32> = line
            .split(",")
            .map(|x| x.parse().unwrap())
            .collect();

        // Correct order
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
            continue;
        }

        // Fix order
        let mut correct_page_update = vec![0; page_update.len()];
        page_update.iter().for_each(|page| {
            let i = prev_pages[page]
                .iter()
                .filter(|y| page_update.contains(y))
                .count();
            correct_page_update[i] = *page;
        });
        sum += correct_page_update[correct_page_update.len() / 2];
    }

    println!("{}", sum);

    Ok(())
}

// use std::{ collections::{ HashMap, HashSet }, error::Error, io };

// fn main() -> Result<(), Box<dyn Error>> {
//     let mut correct_order = vec![];
//     let mut pages_before = HashMap::new();

//     for line in io::stdin().lines() {
//         let line = line.unwrap();
//         if line.is_empty() {
//             break;
//         }

//         let nums: Vec<i32> = line
//             .split("|")
//             .filter_map(|x| x.parse().ok())
//             .collect();

//         *pages_before.entry(nums[1]).or_insert(0) += 1;
//         pages_before.entry(nums[0]).or_insert(0);
//     }
//     let size = pages_before.len();
//     correct_order.resize(size, 0);
//     pages_before.iter().for_each(|(page, order)| {
//         correct_order[*order as usize] = *page;
//     });
//     let correct_order = correct_order;

//     let mut sum = 0;
//     for line in io::stdin().lines() {
//         let line = line.unwrap();
//         if line.is_empty() {
//             break;
//         }
//         let page_update: Vec<i32> = line
//             .split(",")
//             .map(|x| x.parse().unwrap())
//             .collect();

//         let mut is_correct = false;
//         let mut i = 0;
//         let mut j = 0;
//         loop {
//             if j == page_update.len() {
//                 is_correct = true;
//                 break;
//             }
//             if i == correct_order.len() {
//                 break;
//             }
//             if correct_order[i] == page_update[j] {
//                 i += 1;
//                 j += 1;
//             } else {
//                 i += 1;
//             }
//         }
//         if is_correct {
//             continue;
//         }

//         let correct_update: Vec<&i32> = correct_order
//             .iter()
//             .filter(|x| page_update.contains(*x))
//             .collect();
//         sum += correct_update[correct_update.len() / 2];
//     }
//     println!("{}", sum);

//     Ok(())
// }
