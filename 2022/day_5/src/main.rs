use std::fs;

//const FILE: &str = "test.txt";
const FILE: &str = "final.txt";

struct Warehouse {
    stacks: Vec<Vec<char>>,
}

#[derive(Debug)]
struct Manipulation {
    src: usize,
    dest: usize,
    rep: usize,
}

impl Warehouse {
    fn new(num_stacks: usize) -> Self {
        let mut stacks: Vec<Vec<char>> = Vec::new();
        for _ in 0..num_stacks {
            stacks.push(Vec::new());
        }

        Self { stacks }
    }

    fn push(&mut self, stack: usize, cargo: char) {
        self.stacks[stack].push(cargo);
    }

    fn to_char_idx(stack: usize) -> usize {
        stack * 4 + 1
    }

    fn pop(&mut self, stack: usize) -> char {
        self.stacks[stack].pop().unwrap()
    }

    fn reloc(&mut self, source: usize, dest: usize) {
        let cargo = self.pop(source);
        self.push(dest, cargo);
    }

    fn manip(&mut self, m: &Manipulation) {
        let mut crane_load: Vec<char> = Vec::new();
        for _ in 0..m.rep {
            crane_load.push(self.pop(m.src));
        }

        for cargo in crane_load.into_iter().rev() {
            self.push(m.dest, cargo);
        }
    }

    fn top(&self) -> String {
        let mut top_repr = String::new();
        for stack in self.stacks.iter() {
            top_repr.push(*stack.last().unwrap());
        }

        top_repr
    }
}

impl Manipulation {
    fn new(src: usize, dest: usize, rep: usize) -> Self {
        Manipulation{ src, dest, rep }
    }
}

fn main() {
    let contents = fs::read_to_string(FILE).unwrap();
    let lines = contents.lines();

    let mut stack_repr: Vec<&str> = Vec::new();
    let mut manips: Vec<Manipulation> = Vec::new();

    let mut parse_init_cond: bool = true;
    for line in lines {
        if parse_init_cond {
            // First, parse the warehouse's initial condition
            
            // If we hit an empty line, then we've taken in the complete initial 
            // condition for the stack.
            if line.trim().is_empty() { 
                parse_init_cond = false; 
                continue;
            }

            println!("{}", line);
            stack_repr.push(line);
        } else {
            // Now parse the stack manipulations
            println!("now the movements");
            let mut manip_str = line.split_whitespace();
            manip_str.next();
            let rep: usize = manip_str.next().unwrap().parse::<usize>().unwrap();
            manip_str.next();
            let src: usize = manip_str.next().unwrap().parse::<usize>().unwrap() - 1;
            manip_str.next();
            let dest: usize = manip_str.next().unwrap().parse::<usize>().unwrap() - 1;
            
            manips.push(
                Manipulation::new(
                    src, dest, rep
                )
            );
        }
    }

    let num_stacks = (stack_repr.last().unwrap().len() + 1) / 4;
    println!("num stacks: {}", num_stacks);

    let mut wh = Warehouse::new(num_stacks);

    let mut stack_load = stack_repr.into_iter().rev();
    _ = stack_load.next();

    for line in stack_load {
        for i in 0..num_stacks {
            let next_char: char = line.as_bytes()[Warehouse::to_char_idx(i)] as char;
            if next_char != ' ' {
                wh.push(i, next_char);
            }
        }
    }

    for m in manips.iter() {
        wh.manip(m);
    }
     
    let top_cargo = wh.top();

    println!("{}", top_cargo);
}
