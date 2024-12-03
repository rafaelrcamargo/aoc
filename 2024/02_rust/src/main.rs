use std::fs;

enum Direction {
    Increasing,
    Decreasing,
    Unkown,
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

    let mut safe = 0;
    'outer: for mut line in lines {
        println!("{:?}", line);

        let mut direction = Direction::Unkown;
        let mut prev = line[0];

        'inner: for e in line[1..].iter() {
            println!("{} {}", e, prev);

            let diff = e - prev;

            if diff == 0 {
                println!("Numbers are the same {} {}", e, prev);
                continue 'outer;
            }
            if diff.abs() > 3 {
                println!("Difference is too big {}", diff);
                continue 'outer;
            }

            match direction {
                Direction::Unkown => {
                    if diff > 0 {
                        direction = Direction::Increasing;
                    } else if diff < 0 {
                        direction = Direction::Decreasing;
                    } else {
                        println!("Numbers are the same {} {}", e, prev);
                        continue 'outer;
                    }
                }
                Direction::Increasing => {
                    if diff < 0 {
                        println!("Numbers are not increasing anymore");
                        continue 'outer;
                    }
                }
                Direction::Decreasing => {
                    if diff > 0 {
                        println!("Numbers are not decreasing anymore");
                        continue 'outer;
                    }
                }
            }

            prev = e.to_owned();
        }

        safe += 1;
    }

    println!("Part One: {}", safe);
}
