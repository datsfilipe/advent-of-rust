use std::fs;

fn read () -> String {
    String::from_utf8_lossy(&fs::read("src/02/input.txt").unwrap()).parse().unwrap()
}

pub fn solution_for_02 () {
    let strategies = read();
    let round_strategies = strategies.split("\n").collect::<Vec<&str>>();
    let mut acc: u32 = 0;
    for strategy in round_strategies {
        if strategy == "" {
            break;
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
    }
    println!("solution_for_02: {}", acc);
}

pub fn solution_for_02_pt_2 () {
    let strategies = read();
    let round_strategies = strategies.split("\n").collect::<Vec<&str>>();
    let mut acc: u32 = 0;
    for strategy in round_strategies {
        if strategy == "" {
            break;
        }
        let moves = strategy.split(" ").collect::<Vec<&str>>();
        // I did that and I'm not proud
        if moves[1] == "X" {
            acc = acc + 0;
            if moves[0] == "A" {
                acc = acc + 3;
            }
            if moves[0] == "B" {
                acc = acc + 1;
            }
            if moves[0] == "C" {
                acc = acc + 2;
            }
        } else if moves[1] == "Y" {
            acc = acc + 3;
            if moves[0] == "A" {
                acc = acc + 1;
            }
            if moves[0] == "B" {
                acc = acc + 2;
            }
            if moves[0] == "C" {
                acc = acc + 3;
            }
        } else {
            acc = acc + 6;
            if moves[0] == "A" {
                acc = acc + 2;
            }
            if moves[0] == "B" {
                acc = acc + 3;
            }
            if moves[0] == "C" {
                acc = acc + 1;
            }
        }
    }
    println!("solution_for_02_pt_2: {}", acc);
}
