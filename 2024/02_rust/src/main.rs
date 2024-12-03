use std::fs;

enum Direction {
    Increasing,
    Decreasing,
    Unknown,
}

fn is_sequence_safe(sequence: &[i32]) -> bool {
    if sequence.len() < 2 {
        return true;
    }

    let mut direction = Direction::Unknown;
    let mut prev = sequence[0];

    for &curr in &sequence[1..] {
        let diff = curr - prev;

        if diff == 0 || diff.abs() > 3 {
            return false;
        }

        match direction {
            Direction::Unknown => {
                direction = if diff > 0 {
                    Direction::Increasing
                } else {
                    Direction::Decreasing
                };
            }
            Direction::Increasing => {
                if diff < 0 {
                    return false;
                }
            }
            Direction::Decreasing => {
                if diff > 0 {
                    return false;
                }
            }
        }

        prev = curr;
    }

    true
}

fn main() {
    let input = fs::read_to_string("assets/input.txt").unwrap();

    let lines = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    // Part One
    let mut safe = 0;
    for line in lines.clone() {
        if is_sequence_safe(&line) {
            safe += 1;
            continue;
        }
    }
    println!("Part One: {}", safe);

    // Part Two
    let mut safe = 0;
    'outer: for line in lines {
        // First check if the sequence is already safe
        if is_sequence_safe(&line) {
            safe += 1;
            continue;
        }

        // Try removing one number at a time and check if the resulting sequence is safe
        for i in 0..line.len() {
            let mut modified_line = line.clone();
            modified_line.remove(i);

            if is_sequence_safe(&modified_line) {
                safe += 1;
                continue 'outer;
            }
        }
    }
    println!("Part Two: {}", safe);
}
