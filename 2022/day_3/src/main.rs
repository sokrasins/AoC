use std::fs;

//const FILE: &str = "test.txt";
const FILE: &str = "final.txt";

fn score(c: char) -> u32 {
    let ord: u32 = c.into();
    return if ord >= 97 {
        ord - 97 + 1
    } else {
        ord - 65 + 26 + 1
    }
}

// fn main() {
//     let contents = fs::read_to_string(FILE).unwrap();
//     let lines = contents.lines();

//     let mut sum: u32 = 0;
//     for line in lines {
//         let (first, last) = line.split_at(line.len()/2);
//         let intersect: char = first.chars().filter(|c| last.contains(*c)).collect::<Vec<char>>()[0];
//         sum += score(intersect);
//     }

//     println!("total score: {}", sum);
// }

fn main() {
    let contents = fs::read_to_string(FILE).unwrap();
    let num_groups = contents.lines().count() / 3;
    let mut lines = contents.lines();

    let mut sum: u32 = 0;
    for _ in 0..num_groups {
        let line1 = lines.next().unwrap();
        let line2 = lines.next().unwrap();
        let line3 = lines.next().unwrap();

        let intersect = line1.chars().filter(|c| line2.contains(*c)).collect::<Vec<char>>();
        let intersect = intersect.into_iter().filter(|c| line3.contains(*c)).collect::<Vec<char>>();

        println!("{:?}", intersect[0]);
        sum += score(intersect[0]);
    }

    println!("total score: {}", sum);



}
