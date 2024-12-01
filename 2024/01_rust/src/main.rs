use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("assets/input.txt").unwrap();

    let mut lists = vec![vec![], vec![]];
    input.lines().for_each(|line| {
        line.split_whitespace()
            .enumerate()
            .for_each(|(i, n)| lists[i].push(n.parse::<i32>().unwrap()));
    });
    lists.iter_mut().for_each(|list| list.sort());

    println!(
        "Part One: {}",
        lists[0]
            .iter()
            .enumerate()
            .fold(0, |acc, (i, n)| { acc + (n - lists[1][i]).abs() })
    );

    let mut occurrences: HashMap<i32, i32> = HashMap::new();
    lists[1].iter().for_each(|n| {
        occurrences.entry(*n).and_modify(|e| *e += 1).or_insert(1);
    });

    println!(
        "Part Two: {}",
        lists[0]
            .iter()
            .fold(0, |acc, n| acc + (n * occurrences.get(n).unwrap_or(&0)))
    );
}
