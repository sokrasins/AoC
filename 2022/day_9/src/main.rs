use std::fs;
use std::ops::{Add, AddAssign, Sub};
use std::str::FromStr;

//const FILE: &str = "test.txt";
//const FILE: &str = "test2.txt";
const FILE: &str = "final.txt";

#[derive(Debug, Copy, Clone, Eq, Ord, PartialOrd)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        (self.x == other.x) && (self.y == other.y)
    }
}

struct Rope {
    knots: Vec<Point>,
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(input: &str) -> Result<Direction, Self::Err> {
        match input {
            "U" => Ok(Direction::Up),
            "D" => Ok(Direction::Down),
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            _   => Err(()),
        }
    }
}

impl Rope {
    fn new(x: i32, y: i32, num_knots: usize) -> Self {
        let mut knots = Vec::new();
        knots.resize(num_knots, Point::new(x, y));
        return Self { knots }
    }

    fn move_head(&mut self, dir: &Direction) {
        match dir {
            Direction::Up => { self.knots[0].y += 1; },
            Direction::Down => { self.knots[0].y -= 1; },
            Direction::Left => { self.knots[0].x -=1; },
            Direction::Right => { self.knots[0].x += 1; },
        }

        for i in 1..self.knots.len() {
            self.update(i);
        }
    }

    fn update(&mut self, idx: usize) {
        let diff = self.knots[idx-1] - self.knots[idx];  
        
        if (diff.x.abs() <= 1) && (diff.y.abs() <= 1) {
            return;
        }
        
        let change = Point{ 
            x: diff.x.clamp(-1, 1), 
            y: diff.y.clamp(-1, 1), 
        };

        self.knots[idx] += change;
    }
}

fn main() {
    let contents = fs::read_to_string(FILE).unwrap();
    let lines = contents.lines();

    let mut history: Vec<Point> = Vec::new();
    let mut r = Rope::new(0, 0, 10);

    for line in lines {
        println!("{}", line);
        let mut movement = line.split_whitespace();
        let dir = Direction::from_str(movement.next().unwrap()).unwrap();
        let num: usize = movement.next().unwrap().parse().unwrap();

        for _ in 0..num {
            r.move_head(&dir);
            history.push(*r.knots.last().unwrap());
        }
    }

    for p in history.iter() {
        println!("{:?}", p);
    }

    history.sort();
    history.dedup();
    println!("");

    for p in history.iter() {
        println!("{:?}", p);
    }

    println!("num visits: {}", history.len());
}
