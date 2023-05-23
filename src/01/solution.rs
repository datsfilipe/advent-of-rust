use std::fs;

fn read () -> String {
    String::from_utf8_lossy(&fs::read("src/01/input.txt").unwrap()).parse().unwrap()
}

pub fn solution_for_01 () {
    let mut highest: u32 = 0;

    let calories_string = read();
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

pub fn solution_for_01_pt_2 () {
    let mut highest_three: Vec<u32> = vec![0; 3];

    let calories_string = read();
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
        for index in 0..highest_three.len() {
            let min_value = highest_three.as_slice().iter().min();
            if !highest_three.as_slice().iter().any(|&i| i == acc) {
                if highest_three[index] == *min_value.unwrap() {
                    if highest_three[index] < acc {
                        highest_three[index] = acc;
                    }
                }
            }
        }
    }
    println!("solution_for_01_pt_2: {}", highest_three[0] + highest_three[1] + highest_three[2]);
}
