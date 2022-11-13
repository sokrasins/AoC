use std::fs;
//use std::vec::Vec;

const FILE: &str = "test.txt";
//const FILE: &str = "final.txt";

fn main() {
    let contents = fs::read_to_string(FILE).unwrap();
    let mut lines = contents.lines();
}
