use std::fs;

//const FILE: &str = "test.txt";
const FILE: &str = "final.txt";

// Start of packet length
const SOP_LEN: usize = 4;

// Start of message length
const SOM_LEN: usize = 14;

fn all_unique_chars(s: &str) -> bool {
    let mut s_temp: Vec<char> = s.chars().collect();
    s_temp.sort();

    for i in 0..(s_temp.len() - 1) {
        if s_temp[i] == s_temp[i+1] { return false; }
    }

    true
}

fn main() {
    let contents = fs::read_to_string(FILE).unwrap();
    let lines = contents.lines();

    for line in lines {
        println!("{}", line);

        if line.len() < SOM_LEN {
            println!("Invalid line length, skipping...");
            continue;
        }

        for i in 0..line.len() {
            if all_unique_chars(&line[i..i+SOM_LEN]) {
                println!("Done at i={}", i+SOM_LEN);
                break;
            }
        }
    }
}
