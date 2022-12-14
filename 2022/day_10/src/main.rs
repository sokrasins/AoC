use std::fs;

//const FILE: &str = "test.txt";
const FILE: &str = "final.txt";

const LINES_PER_SCREEN: usize = 6;
const CHARS_PER_LINE: usize = 40;

#[derive(Clone)]
struct State {
    x: i32,
    cycle: u32,
}

fn x(system: &Vec<State>, cycle: u32) -> i32 {
    system[(cycle-1) as usize].x
}

fn main() {
    let contents = fs::read_to_string(FILE).unwrap();
    let lines = contents.lines();

    let mut system: Vec<State> = Vec::new();
    system.push( State { x: 1, cycle: 1 } );

    for line in lines {
        println!("{}", line);
        
        let mut instr = line.split_whitespace();
        let mut last_state = system.last().unwrap().clone();
        match instr.next().unwrap() {
            "noop" => {
                last_state.cycle += 1;
                system.push(last_state);
            },
            "addx" => {
                let val: i32 = instr.next().unwrap().parse().unwrap();
                last_state.cycle += 1;  
                system.push(last_state.clone());
                last_state.x += val;
                last_state.cycle += 1;
                system.push(last_state);
            },
            _ => {},
        }
    }

    let score_list: [u32; 6] = [20, 60, 100, 140, 180, 220];

    let mut total_score: i32 = 0;
    for i in score_list {
        let score = (i as i32) * x(&system, i);
        total_score += score;
        println!("{}: {}", i, score);
    }

    println!("total: {}", total_score);

    for j in 0..LINES_PER_SCREEN {
        for i in 0..CHARS_PER_LINE {
            let sprite_loc = system[i + (j*CHARS_PER_LINE)].x;
            let display_char = if sprite_loc == (i as i32) || sprite_loc-1 == (i as i32) || sprite_loc+1 == (i as i32) {
                "#"
            } else {
                "."
            };
            print!("{}", display_char);
        }
        println!("");
    }

}
