use std::fs;

const FILE: &str = "test.txt";
//const FILE: &str = "final.txt";

fn main() {
    let contents = fs::read_to_string(FILE).unwrap();
    let lines = contents.lines();

    for line in lines {
        println!("{}", line);
    }
}
