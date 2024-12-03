fn parse_line(hay: &str) -> i32 {
    regex::Regex::new(r"mul\(([0-9]+),([0-9]+)\)")
        .unwrap()
        .captures_iter(hay)
        .map(|c| c.extract())
        .map(|(_, [multiplicand, multiplier])| {
            multiplicand.parse::<i32>().unwrap() * multiplier.parse::<i32>().unwrap()
        })
        .sum::<i32>()
}

fn main() {
    let input = std::fs::read_to_string("assets/input.txt").unwrap();
    println!("Part One: {}", parse_line(&input));
    println!(
        "Part Two: {}",
        input
            .split("don't()")
            .enumerate()
            .map(|(i, s)| {
                s.split_once("do()")
                    .unwrap_or(if i == 0 { ("", s) } else { (s, "") })
            })
            .map(|(_, line)| parse_line(line))
            .sum::<i32>()
    );
}
