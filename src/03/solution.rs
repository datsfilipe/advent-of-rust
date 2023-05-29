use std::fs;

fn read () -> String {
    String::from_utf8_lossy(&fs::read("src/03/input.txt").unwrap()).parse().unwrap()
}

fn priority (letter: &str) -> usize {
    let alphabet = "abcdefghijklmnopqrstuvwxyz".split("").collect::<Vec<&str>>();
    let index: usize;

    if letter == letter.to_uppercase().as_str() {
        index = alphabet.iter().position(|x| x.to_uppercase() == letter).unwrap();
        return index + 26;
    }

    index = alphabet.iter().position(|x| *x == letter).unwrap();
    return index;
}

pub fn solution_for_03 () {
    let input = read();
    let ruckstack = input.split("\n").collect::<Vec<&str>>();
    let mut priority_sum: usize = 0;

    for i in 0..ruckstack.as_slice().len() {
        if &ruckstack[i] == &"" {
            break;
        }
        let halfs = &ruckstack[i].split_at(&ruckstack[i].len()/2);
        match halfs {
            (x, y) => {
                let listx = &x.split("").collect::<Vec<&str>>();
                let listy = &y.split("").collect::<Vec<&str>>();
                // the lists have two empty strings in the beginning and the end, why?
                for f in 0..listx.len() {
                    if listy.contains(&listx[f]) && listx[f] != "" {
                        priority_sum += priority(listx[f]);
                        break;
                    }
                }
            },
        }
    }

    println!("solution_for_03_pt1: {}", priority_sum);
}
