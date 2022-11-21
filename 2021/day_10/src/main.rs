use std::fs;
//use std::vec::Vec;

//const FILE: &str = "test.txt";
const FILE: &str = "final.txt";

#[derive(Clone, Copy, PartialEq, Debug)]
enum Type {
    Paren,
    Bracket,
    Brace,
    Angle,
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Closure {
    Open(Type),
    Close(Type),
}

impl Closure {
    pub fn from_char(c: char) -> Result<Self, ()> {
        return match c {
            '(' => Ok(Closure::Open(Type::Paren)),
            '[' => Ok(Closure::Open(Type::Bracket)),
            '{' => Ok(Closure::Open(Type::Brace)),
            '<' => Ok(Closure::Open(Type::Angle)),

            ')' => Ok(Closure::Close(Type::Paren)),
            ']' => Ok(Closure::Close(Type::Bracket)),
            '}' => Ok(Closure::Close(Type::Brace)),
            '>' => Ok(Closure::Close(Type::Angle)),

            _ => Err(()),
        }
    }

    pub fn score_error(&self) -> u32 {
        return match &self {
            Closure::Open(_) => 0,
            Closure::Close(t) => {
                return match t {
                    Type::Paren => 3,
                    Type::Bracket => 57,
                    Type::Brace => 1197,
                    Type::Angle => 25137,
                }
            }
        }
    }

    pub fn score_incomplete(&self) -> u64 {
        return match &self {
            Closure::Open(_) => 0,
            Closure::Close(t) => {
                return match t {
                    Type::Paren => 1,
                    Type::Bracket => 2,
                    Type::Brace => 3,
                    Type::Angle => 4,
                }
            }
        }
    }
}

#[derive(Debug)]
struct ParseStack {
    stack: Vec<Closure>,
}

impl ParseStack {
    pub fn new() -> Self {
        ParseStack {
            stack: Vec::new(),
        }
    }

    pub fn push(&mut self, c: &Closure) -> Result<(), Option<Closure>> {
        if let Closure::Close(new_t) = c {
            let last_t = self.stack.last();
            return match last_t {
                None => Err(None),
                Some(Closure::Close(_)) => Err(None),
                Some(Closure::Open(last_open)) => if *last_open == *new_t {
                    _ = self.stack.pop();
                    return Ok(());
                } else {
                    return Err(Some(Closure::Open(*last_open)));
                },
            } 
        } else {
            self.stack.push(*c); 
            Ok(())
        }
    }
}

fn main() {
    let contents = fs::read_to_string(FILE).unwrap();
    let lines = contents.lines();

    let mut error_score: u32 = 0;
    let mut incomplete_score: Vec<u64> = Vec::new();
    'codeline: for line in lines {
        let mut ps = ParseStack::new();

        for c in line.chars() {
            let token = Closure::from_char(c).unwrap();
            match ps.push(&token) {
                Ok(()) => (),
                Err(None) => {
                    println!("uh oh, no expected character given");
                    continue 'codeline;
                },
                Err(Some(_)) => {
                    println!("Illegal char: {:?}", token);
                    error_score += token.score_error(); 
                    continue 'codeline;
                },
            }
        }

        println!("{:?}", ps);        
        // if we're here, the line is incomplete and not erroneous.
        let mut line_score: u64 = 0;
        while ps.stack.len() > 0 {
            let last_token = ps.stack.last().unwrap();
            match last_token {
                Closure::Open(t) => {
                    let repair_token = Closure::Close(*t);
                    line_score *= 5;
                    line_score += repair_token.score_incomplete();
                    _ = ps.push(&repair_token);
                }
                _ => { println!("Error: unexpected token"); },
            }
        }
        incomplete_score.push(line_score);
    }

    incomplete_score.sort();

    let middle_idx = incomplete_score.len() / 2;

    println!("error line score: {}", error_score);
    println!("autocomplete score: {:?}", incomplete_score[middle_idx]);
}
