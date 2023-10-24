use std::fs;

struct List(Vec<i32>);
impl List {
    fn sort(&mut self) -> List {
        let mut list = self.0.clone();
        list.sort();
        list.reverse();
        List(list)
    }
}

impl FromIterator<i32> for List {
    fn from_iter<I: IntoIterator<Item = i32>>(iter: I) -> Self {
        let mut list = Vec::new();
        for i in iter {
            list.push(i);
        }
        List(list)
    }
}

fn calories(elves: String) -> List {
    elves
        .split("\n\n")
        .map(|inventory| {
            inventory
                .split('\n')
                .map(|food| food.parse().unwrap_or(0))
                .sum()
        })
        .collect::<List>()
}

fn top_caloric_elves(elves: String, top: usize) -> i32 {
    calories(elves).sort().0.iter().take(top).sum()
}

pub fn dirty() {
    /* Challenge 1 */
    let elves = fs::read_to_string("assets/input.txt").unwrap();
    println!("First Elf: {}", top_caloric_elves(elves, 1));

    /* Challenge 2 */
    let elves = fs::read_to_string("assets/input.txt").unwrap();
    println!("Top 3 Elves: {}", top_caloric_elves(elves, 3))
}
