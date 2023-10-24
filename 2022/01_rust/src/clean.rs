use std::fs;

fn get_calories(elves: String) -> Vec<i32> {
    elves
        .split("\n\n")
        .map(|inventory| {
            inventory
                .split('\n')
                .map(|food| food.parse().unwrap_or(0))
                .sum()
        })
        .collect::<Vec<i32>>()
}

fn top_caloric_elves(elves: String, top: usize) -> i32 {
    // Save
    let mut calories = get_calories(elves);

    // Mutate
    calories.sort();
    calories.reverse();

    // Return
    calories.iter().take(top).sum()
}

pub fn clean() {
    /* Challenge 1 */
    let elves = fs::read_to_string("assets/input.txt").unwrap();
    println!("First Elf: {}", top_caloric_elves(elves, 1));

    /* Challenge 2 */
    let elves = fs::read_to_string("assets/input.txt").unwrap();
    println!("Top 3 Elves: {}", top_caloric_elves(elves, 3))
}
