use std::fs;

//const FILE: &str = "test.txt";
const FILE: &str = "final.txt";

fn main() {
    let contents = fs::read_to_string(FILE).unwrap();
    let lines = contents.lines();

    let mut calories: Vec<u32> = Vec::new();

    let mut cur_count: u32 = 0;
    for line in lines {
        match line.parse::<u32>() {
            Ok(n) => { cur_count += n; },
            Err(_) => { 
                calories.push(cur_count);
                cur_count = 0;
            }
        }
    }
    calories.push(cur_count);

    calories.sort_by(|a, b| b.partial_cmp(a).unwrap());

    let max_cals = calories[0];

    let top_three = calories[0] +
            calories[1] +
            calories[2];

    println!("{:?}", calories);
    println!("max calories: {:?}", max_cals);
    println!("sum of top three: {:?}", top_three);
}
