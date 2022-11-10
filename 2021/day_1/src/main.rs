use std::fs;
use std::collections::VecDeque;

//const FILE: &str = "test.txt";
const FILE: &str = "final.txt";

struct Filter {
    map: VecDeque<i32>,
    kernel_size: usize,
}

impl Filter {
    pub fn new(kernel_size: usize) -> Filter {
        Filter {
            map: VecDeque::new(),
            kernel_size,
        }
    }

    pub fn apply(&mut self, new_depth: i32) -> Option<i32> {
        self.push(new_depth);    
        self.sum()
    }

    fn push(&mut self, new_depth: i32) {
        self.map.push_front(new_depth);
        while self.map.len() > self.kernel_size {
            self.map.pop_back();
        }
    }


    fn sum(&self) -> Option<i32> {
        if self.map.len() >= self.kernel_size {
            let mut sum: i32 = 0; 
            for val in &self.map {
                sum += val;
            }
            return Some(sum);
        }
        None
    }
}

fn find_num_descents(data: &String, kern: usize) -> u32 {
    let mut f = Filter::new(kern);

    let mut last_depth: i32 = std::i32::MAX;
    let mut total_increase: u32 = 0;
    for line in data.lines() {
        let depth: i32 = line.parse().unwrap();

        if let Some(smooth_depth) = f.apply(depth) {
            if smooth_depth > last_depth {
                total_increase += 1;
            }
            last_depth = smooth_depth;
        }
    }
    total_increase
}

fn main() -> Result<(), String> {
    let contents = fs::read_to_string(FILE)
        .expect("Should have been able to read the file");

    let ans = find_num_descents(&contents, 1);
    println!("Part 1 answer: {ans}");

    let ans = find_num_descents(&contents, 3);
    println!("Part 2 answer: {ans}");

    Ok(())
}
