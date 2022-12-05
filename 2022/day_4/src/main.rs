use std::fs;

//const FILE: &str = "test.txt";
const FILE: &str = "final.txt";

struct Range {
    start: i32, 
    end: i32,
}

impl Range {
    fn new(start: i32, end: i32) -> Range {
        Range { start, end }
    }

    fn from_range_str(repr: &str) -> Range {
        let mut range_nums = repr.split('-');

        Self::new(
            range_nums.next().unwrap().parse().unwrap(),
            range_nums.next().unwrap().parse().unwrap()
        )
    }

    fn contains(&self, other: &Self) -> bool {
        other.start >= self.start && other.end <= self.end
    }

    fn overlaps(&self, other: &Self) -> bool {
        let mut self_range = self.start..self.end+1;
        let other_range = other.start..other.end+1;
        
        self_range.any(|i| other_range.contains(&i))
    }
}

fn main() {
    let contents = fs::read_to_string(FILE).unwrap();
    let lines = contents.lines();

    let mut contains_sum: u32 = 0;
    let mut overlaps_sum: u32 = 0;

    for line in lines {
        let mut split_iter = line.split(',');
        let first = Range::from_range_str(split_iter.next().unwrap());
        let second = Range::from_range_str(split_iter.next().unwrap()); 

        let contains = first.contains(&second) || second.contains(&first);
        let overlaps = first.overlaps(&second);
        contains_sum += if contains { 1 } else { 0 };
        overlaps_sum += if overlaps { 1 } else { 0 };
        println!("contains: {}", contains);
        println!("overlaps: {}", overlaps);
    }

    println!("{}", contains_sum);
    println!("{}", overlaps_sum);
}
