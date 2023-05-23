use std::fs;

pub fn solution_for_01 () {
    let mut highest: u32 = 0;

    let calories_string: String = String::from_utf8_lossy(&fs::read("src/01/input.txt").unwrap()).parse().unwrap();
    let parts = calories_string.split("\n\n");
    let calories_array = parts.collect::<Vec<&str>>();
    for item in calories_array {
        let item_parts = item.split("\n");
        let item_array = item_parts.collect::<Vec<&str>>();
        let mut acc: u32 = 0;
        for item in &item_array {
            if item == &"" {
                break;
            }
            acc = acc + item.parse::<u32>().unwrap();
        }
        if acc > highest {
            highest = acc;
        }
    }
    println!("solution_for_01: {}", highest);
}
