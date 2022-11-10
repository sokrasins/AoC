use std::fs;
use std::vec::Vec;

//const FILE: &str = "test.txt";
const FILE: &str = "final.txt";

struct Diagnostic {
    ones: Vec<usize>,
    zeros: Vec<usize>,
}

impl Diagnostic {
    pub fn new(num_digits: usize) -> Self {
        let mut ones: Vec<usize> = Vec::new();
        let mut zeros: Vec<usize> = Vec::new();

        ones.resize(num_digits, 0);
        zeros.resize(num_digits, 0);

        Diagnostic { ones, zeros }
    }

    pub fn from_reading(line: &str) -> Self {
        let mut diagnostic = Self::new(line.len());
        diagnostic.add_reading(line);

        diagnostic
    }

    pub fn add_reading(&mut self, line: &str) {
        for (i, char) in line.chars().rev().enumerate() {
            match char {
                '0' => self.zeros[i] += 1,
                '1' => self.ones[i] += 1,
                _ => ()
            }
        }
    }

    pub fn gamma(&mut self) -> u32 {
        let mut gamma: u32 = 0;
        for i in 0..self.ones.len() {
            if self.ones[i] > self.zeros[i] {
                gamma |= 1 << i;
            }
        }    
        gamma
    }

    pub fn epsilon(&mut self) -> u32 {
        let gamma = self.gamma();
        let mask: u32 = (2 as u32).pow(self.ones.len() as u32) - 1;

        gamma ^ mask
    }
}

fn calc_diagnostic(readings: &str) -> (u32, u32) {
    let mut line_iter = readings.lines();
    
    let mut diagnostic = Diagnostic::from_reading(line_iter.next().unwrap());
    for line in line_iter {
        diagnostic.add_reading(line);
    }

    (diagnostic.gamma(), diagnostic.epsilon())
}

fn main() {
    let contents = fs::read_to_string(FILE).unwrap();    
    let (gamma, epsilon) = calc_diagnostic(&contents);
    println!("power: {}", gamma * epsilon);
}
