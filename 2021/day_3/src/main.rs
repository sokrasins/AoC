use std::fs;
use std::vec::Vec;

//const FILE: &str = "test.txt";
const FILE: &str = "final.txt";

struct Diagnostic {
    width: usize,
    ones: Vec<usize>,
    zeros: Vec<usize>,
    readings: Vec<u32>,
}

impl Diagnostic {
    pub fn new(num_digits: usize) -> Self {
        let mut ones: Vec<usize> = Vec::new();
        let mut zeros: Vec<usize> = Vec::new();

        ones.resize(num_digits, 0);
        zeros.resize(num_digits, 0);

        Diagnostic { 
            width: num_digits,
            ones,
            zeros,
            readings: Vec::new(),
        }
    }

    pub fn from_reading(line: &str) -> Self {
        let mut diagnostic = Self::new(line.len());
        diagnostic.add_reading(line);

        diagnostic
    }

    pub fn add_reading(&mut self, line: &str) {
        self.readings.push(u32::from_str_radix(line, 2).unwrap());
        // for (i, char) in line.chars().rev().enumerate() {
        //     match char {
        //         '0' => self.zeros[i] += 1,
        //         '1' => self.ones[i] += 1,
        //         _ => ()
        //     }
        // }
    }

    fn dom_bit(list: &Vec<u32>, pos: u32) -> u32 {
        let mask: u32 = 1 << pos;
        let mut ones: u32 = 0;
        let mut zeros: u32 = 0;

        for r in list.iter() {
            if (r & mask) == mask {
                ones += 1;
            } else {
                zeros += 1;
            }
        }

        (ones >= zeros) as u32
    }

    pub fn gamma(&mut self) -> u32 {
        let mut gamma: u32 = 0;
        for i in 0..self.width {
            gamma |= (Diagnostic::dom_bit(&self.readings, i as u32)) << i;
        }
        gamma
    }

    pub fn epsilon(&mut self) -> u32 {
        let gamma = self.gamma();
        let mask: u32 = (2 as u32).pow(self.ones.len() as u32) - 1;

        gamma ^ mask
    }

    fn env_sensor(&mut self, invert: bool) -> Option<u32> {
        let mut l = self.readings.clone();

        for i in 0..self.width {
            let bit_pos: u32 = (self.width-1-i) as u32; 
            let mcb = Diagnostic::dom_bit(&l, bit_pos) ^ (invert as u32);
            l = l
                .into_iter()
                .filter(|x| ((x >> bit_pos) & 1) == mcb)
                .collect();
            // Check for only one
            if l.len() == 1 {
                return Some(l[0]);
            }
        }
        None
    }

    pub fn oxygen(&mut self) -> Option<u32> {
        self.env_sensor(false)    
    }

    pub fn co2(&mut self) -> Option<u32> {
        self.env_sensor(true)
    }
}

fn calc_diagnostic(readings: &str) -> (u32, u32, u32, u32) {
    let mut line_iter = readings.lines();
    
    let mut diagnostic = Diagnostic::from_reading(line_iter.next().unwrap());
    for line in line_iter {
        diagnostic.add_reading(line);
    }

    (
        diagnostic.gamma(), 
        diagnostic.epsilon(), 
        diagnostic.oxygen().unwrap(), 
        diagnostic.co2().unwrap(),
    )
}

fn main() {
    let contents = fs::read_to_string(FILE).unwrap();    
    let (
        gamma, 
        epsilon,
        oxygen,
        co2,
    ) = calc_diagnostic(&contents);

    println!("gamma:        {}", gamma);
    println!("epsilon:      {}", epsilon);
    println!("oxtgen:       {}", oxygen);
    println!("co2:          {}", co2);
    println!("");
    println!("power:        {}", gamma * epsilon);
    println!("life support: {}", oxygen * co2);
}
