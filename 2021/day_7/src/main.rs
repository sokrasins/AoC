use std::fs;
use std::vec::Vec;
use std::cmp::max;
use std::cmp::min;

//const FILE: &str = "test.txt";
const FILE: &str = "final.txt";

fn cost(d: u32) -> u32 {
    (d+1)*d/2
}

fn main() {
    let contents = fs::read_to_string(FILE).unwrap();
    let mut lines = contents.lines();

    let positions = lines
        .next()
        .unwrap()
        .split(",");

    let mut crab_pos: Vec<u32> = Vec::new();

    let mut pos_max: u32 = 0;
    for pos in positions {
        let pos_num = pos.parse().unwrap();
        pos_max = max(pos_max, pos_num);
        crab_pos.push(pos_num);    
    }

    let mut min_fuel: u32 = std::u32::MAX;
    for i in 0..pos_max {
        let mut fuel_cost: u32 = 0;
        for crab in crab_pos.iter() {
            fuel_cost += if crab > &i { cost(crab - i) } else { cost(i - crab) };
        }
        min_fuel = min(min_fuel, fuel_cost);
    }
    println!("{}", min_fuel);
}
