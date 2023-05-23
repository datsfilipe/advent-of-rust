use std::fs;

pub fn solution_for_02 () {
    let strategy: String = String::from_utf8_lossy(&fs::read("src/02/input.txt").unwrap()).parse().unwrap();
    let round_strategies = strategy.split("\n").collect::<Vec<&str>>();
    let mut acc: u32 = 0;
    for strategy in &round_strategies {
        if strategy == &"" {
            return;
        }
        let moves = strategy.split(" ").collect::<Vec<&str>>();
        // I did that and I'm not proud
        if moves[1] == "X" {
            acc = acc + 1;
            if moves[0] == "A" {
                acc = acc + 3;
            }
            if moves[0] == "C" {
                acc = acc + 6;
            }
        } else if moves[1] == "Y" {
            acc = acc + 2;
            if moves[0] == "B" {
                acc = acc + 3;
            }
            if moves[0] == "A" {
                acc = acc + 6;
            }
        } else {
            acc = acc + 3;
            if moves[0] == "C" {
                acc = acc + 3;
            }
            if moves[0] == "B" {
                acc = acc + 6;
            }
        }
        println!("{}", acc);
    }
    // printing acc here showed nothing huh
}
