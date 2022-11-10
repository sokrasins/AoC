use std::fs;

//const FILE: &str = "test.txt";
const FILE: &str = "final.txt";

pub enum Direction {
    UP(u32),
    DOWN(u32),
    FORWARD(u32),
    BACKWARD(u32),
}

impl Direction {
    fn from_line(str: &str) -> Option<Self> {
        let mut tokens = str.split_whitespace();
        let num_dir_chars = tokens.next().unwrap().len();
        let mag: u32 = tokens.next().unwrap().parse().unwrap();
        match num_dir_chars {
            2 => Some(Direction::UP(mag)),
            4 => Some(Direction::DOWN(mag)),
            7 => Some(Direction::FORWARD(mag)),
            8 => Some(Direction::BACKWARD(mag)),
            _ => None
        }
    }
}

fn sim_sub(directions: &str) -> (i32, i32, i32) {
    let mut horz: i32 = 0;
    let mut vert: i32 = 0;
    let mut aim: i32 = 0;

    for line in directions.lines() {
        if let Some(dir) = Direction::from_line(line) {
            match dir {
                Direction::UP(mag) => {
                    //vert -= mag as i32;
                    aim -= mag as i32;
                },
                Direction::DOWN(mag) => {
                    //vert += mag as i32; 
                    aim += mag as i32;
                },
                Direction::FORWARD(mag) => {
                    horz += mag as i32;
                    vert += aim * (mag as i32);
                },
                Direction::BACKWARD(mag) => horz -= mag as i32,
            }
        };
    }

    return (horz, vert, aim);
}

fn main() {
    let contents = fs::read_to_string(FILE)
        .expect("Should ahve been able to read the file");
    
    let (horz, vert, aim) = sim_sub(&contents);
    println!("horz: {}", horz);
    println!("vert: {}", vert);
    println!("aim: {}", aim);
    println!("Part 2: {}", horz * vert);
}

