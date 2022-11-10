use std::fs;

const FILE: &str = "test.txt";
//const FILE: &str = "final.txt";

fn calc_gamma(readings: &str) -> (u32, u32) {
    for line in readings.lines() {
        let new_val = u32::from_str_radix(line, 2).unwrap();
        println!("{}", new_val);
        println!("{}", line.len());
    }

    (0, 0)
}

fn main() {
    let contents = fs::read_to_string(FILE).unwrap();    
    let (gamma, epsilon) = calc_gamma(&contents);
}
