use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("assets/input.txt").unwrap();
    let data = input.split("\n\n").collect::<Vec<&str>>();

    let pages = data[0]
        .split("\n")
        .map(|x| {
            x.split("|")
                .map(|y| y.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    let mut numbers: HashMap<i32, Vec<i32>> = HashMap::new();
    for page in pages {
        numbers.entry(page[0]).or_insert(Vec::new());
        numbers.entry(page[1]).or_insert(Vec::new());
        numbers.get_mut(&page[1]).unwrap().push(page[0]);
    }

    let updates = data[1]
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| {
            x.split(",")
                .map(|y| y.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    let mut valid_updates: Vec<i32> = vec![];
    let mut invalid_updates: Vec<Vec<i32>> = vec![];
    for update in updates {
        let mut valid = true;

        for i in 0..update.len() {
            let previous = update.get(..i).unwrap();

            if let Some(order) = numbers.get(&update[i]) {
                for p in previous {
                    if !order.contains(p) {
                        valid = false;
                        break;
                    }
                }
            }
        }

        if valid {
            valid_updates.push(update[update.len() / 2]);
        } else {
            invalid_updates.push(update);
        }
    }
    println!("Part One: {:?}", valid_updates.iter().sum::<i32>());

    let mut count = 0;
    for invalid_update in invalid_updates {
        let mut sorted_update = invalid_update.clone();

        sorted_update.sort_by(|a, b| {
            let a_before = numbers
                .get(a)
                .unwrap()
                .iter()
                .filter(|x| invalid_update.contains(x))
                .collect::<Vec<&i32>>();

            let b_before = numbers
                .get(b)
                .unwrap()
                .iter()
                .filter(|x| invalid_update.contains(x))
                .collect::<Vec<&i32>>();

            a_before.len().cmp(&b_before.len())
        });

        count += sorted_update[sorted_update.len() / 2];
    }
    println!("Part Two: {:?}", count);
}
