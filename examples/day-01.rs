use std::fs;

const TOP_ELVES: i32 = 3;

fn main() {
    let mut total_max_calories = 0;
    let file_contents = read_file("./examples/day-01-input.txt");

    let mut total_calories_per_elf_map: Vec<i32> = retrieve_calories_per_elf(file_contents);

    (0..TOP_ELVES).for_each(|_i| {
        let max_calories = total_calories_per_elf_map.iter().max().unwrap();
        let elf_index = total_calories_per_elf_map
            .iter()
            .position(|v| v == max_calories)
            .unwrap();

        total_max_calories += total_calories_per_elf_map.remove(elf_index);
    });

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
        .enumerate()
        .map(|(_elf_index, elf_inventory)| {
            let mut elf_calories = 0;
            elf_inventory
                .split("\n")
                .enumerate()
                .for_each(|(_i, item_calorie)| {
                    let parsed_item_calorie = match item_calorie.parse::<i32>() {
                        Ok(v) => v,
                        Err(_) => 0,
                    };

                    elf_calories += parsed_item_calorie
                });

            elf_calories
        })
        .collect()
}
