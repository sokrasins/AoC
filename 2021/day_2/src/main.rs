use std::fs;

const FILE: &str = "test.txt";
// const FILE: &str = "final.txt";

pub enum Direction {
    UP(u32),
    DOWN(u32),
    FORWARD(u32),
    BACKWARD(u32),
}

impl Direction {
    fn from_line(str: &str) -> Option<Self> {
        // TODO
        Some(Direction::UP(4))
    }
}

fn main() {
    let contents = fs::read_to_string(FILE)
        .expect("Should ahve been able to read the file");
    
    let (horz, vert) = sim_sub(&contents);
    println!("Part 1: {}", horz * vert);
}

fn sim_sub(directions: &str) -> (i32, i32) {
    let mut horz: i32 = 0;
    let mut vert: i32 = 0;

    for line in directions.lines() {
        if let Some(dir) = Direction::from_line(line) {
            match dir {
                Direction::UP(mag) => vert -= mag as i32,
                Direction::DOWN(mag) => vert += mag as i32,
                Direction::FORWARD(mag) => horz += mag as i32,
                Direction::BACKWARD(mag) => horz -= mag as i32,
            }
        };
    }

    return (horz, vert);
}
