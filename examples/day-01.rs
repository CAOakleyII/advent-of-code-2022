use std::fs;

const TOP_ELVES: usize = 3;

fn main() {
    let file_contents = read_file("./examples/day-01-input.txt");

    let mut total_calories_per_elf_map: Vec<i32> = retrieve_calories_per_elf(file_contents);

    total_calories_per_elf_map.sort();

    let total_max_calories: i32 = total_calories_per_elf_map
        .into_iter()
        .rev()
        .take(TOP_ELVES)
        .sum();

    println!("Max Calories: {total_max_calories}")
}

fn read_file(path: &str) -> String {
    let file_read_results = fs::read_to_string(path);
    match file_read_results {
        Ok(file_contents) => file_contents,
        Err(err) => {
            panic!("{:?}", err)
        }
    }
}

fn retrieve_calories_per_elf(input: String) -> Vec<i32> {
    input
        .split("\n\n")
        .map(|elf_inventory| {
            elf_inventory
                .lines()
                .map(|item_calorie| item_calorie.parse::<i32>().unwrap())
        })
        .map(|calories| calories.sum())
        .collect()
}
