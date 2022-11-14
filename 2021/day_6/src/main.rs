use std::fs;

//const FILE: &str = "test.txt";
const FILE: &str = "final.txt";

const SPAWN_TIME: usize = 6;
const BIRTH_DELAY: usize = 2;
const MAX_DAYS: usize = SPAWN_TIME + BIRTH_DELAY;

const STEPS: usize = 256;

fn age(school: &mut [u64]) {
    let temp: u64 = school[0];

    for i in 0..MAX_DAYS {
        school[i] = school[i+1];
    }

    // All fish in slot 0 spawn new fish
    school[MAX_DAYS] = temp;

    // All fish in slot 0 reset their spawn counter
    school[SPAWN_TIME] += temp;
}

fn sum(school: &[u64]) -> u64 {
    let mut tot: u64 = 0;
    for i in school.iter() {
        tot += i; 
    }
    tot
}

fn main() {
    let contents = fs::read_to_string(FILE).unwrap();
    let mut lines = contents.lines();

    let mut school: [u64; MAX_DAYS + 1] = [0; MAX_DAYS + 1];

    let init = lines
        .next()
        .unwrap()
        .split(",");
    
    for i in init {
        school[i.parse::<usize>().unwrap()] += 1;
    }

    for _ in 0..STEPS {
        age(&mut school);
    }

    println!("{}", sum(&school));
}
