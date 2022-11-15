use std::fs;
//use std::vec::Vec;

//const FILE: &str = "test.txt";
const FILE: &str = "final.txt";

fn same(s1: &str, s2: &str) -> bool {
    s1.chars().all(|x| s2.contains(x)) && 
        s2.chars().all(|x| s1.contains(x))
}

fn main() {
    let contents = fs::read_to_string(FILE).unwrap();
    let lines = contents.lines();

    let mut tot: u32 = 0;
    for line in lines {
        // Pull the data out of the line
        let mut enc = line.split("|");
        let digit_enc = enc.next().unwrap();
        let nums = enc.next().unwrap();

        // Sort the digit map by the length of the string
        let mut sorted_enc = digit_enc
            .split_whitespace()
            .collect::<Vec<&str>>();
        sorted_enc.sort_by_key(|a| a.len());

        // Solve the digit mapping
        // Sorted char order: 2 3 4 5 5 5 6 6 6 7
        let mut digit_map: Vec<&str> = Vec::new();
        digit_map.resize(10, "");

        // Get the easy ones first
        digit_map[1] = sorted_enc[0];
        digit_map[7] = sorted_enc[1];
        digit_map[4] = sorted_enc[2];
        digit_map[8] = sorted_enc[9];

        // '3' contains all segments of '7', find it next
        let mut idx_3: usize = 0;
        for i in 3..6 {
            if digit_map[7].chars().all(|x| sorted_enc[i].contains(x)) {
                digit_map[3] = sorted_enc[i];
                idx_3 = i;
            }
        }

        // '9' contains all segments of '3'
        let mut idx_9: usize = 0;
        for i in 6..9 {
            if digit_map[3].chars().all(|x| sorted_enc[i].contains(x)) {
                digit_map[9] = sorted_enc[i];
                idx_9 = i;
            }
        }

        // '5' is contained in '9'
        let mut idx_5: usize = 0;
        for i in 3..6 {
            if i != idx_3 {
                if sorted_enc[i].chars().all(|x| digit_map[9].contains(x)) {
                    digit_map[5] = sorted_enc[i];
                    idx_5 = i;
                }
            }
        }

        // Last digit on size 5 is '2'
        for i in 3..6 {
            if i != idx_3 && i != idx_5 {
                digit_map[2] = sorted_enc[i];    
            }
        }

        // '6' contains '5'
        let mut idx_6: usize = 0;
        for i in 6..9 {
            if i != idx_9 {
                if digit_map[5].chars().all(|x| sorted_enc[i].contains(x)) {
                    digit_map[6] = sorted_enc[i];
                    idx_6 = i;
                }
            }
        }

        // '0' is the last one
        for i in 6..9 {
            if i != idx_6 && i != idx_9 {
                digit_map[0] = sorted_enc[i];
            }
        }

        println!("{:?}", digit_map);

        // Decode the digits
        let mut num: u32 = 0;
        for val in nums.split_whitespace() {
            println!("{:?}", num);
            for (i, enc) in digit_map.iter().enumerate() {
                if same(enc, val) {
                    num *= 10;
                    num += i as u32;
                    break;
                }
            }
        }
        println!("{:?}", num);
        tot += num;
    }
    println!("{}", tot);
}
