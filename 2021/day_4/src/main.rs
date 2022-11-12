use std::fs;
use std::vec::Vec;

//const FILE: &str = "test.txt";
const FILE: &str = "final.txt";

const NUM_ROWS: usize = 5;
const NUM_COLS: usize = 5;

#[derive(Clone, Copy, Debug)]
struct Square {
    value: u32,
    marked: bool,
}

impl Square {
    pub fn new() -> Self {
        Square {
            value: 0,
            marked: false,
        }
    }

    pub fn mark(&mut self) {
        self.marked = true;
    }

    pub fn set_val(&mut self, value: u32) {
        self.value = value;
    }
}

#[derive(Debug)]
struct Board {
    squares: [Square; NUM_ROWS * NUM_COLS],
    finished: bool,
}

impl Board {
    pub fn new() -> Self {
        Board {
            squares: [Square::new(); NUM_ROWS*NUM_COLS],
            finished: false,
        }
    }

    pub fn sq(&mut self, r: usize, c: usize) -> &mut Square {
        &mut self.squares[r + (NUM_ROWS*c)]
    }

    pub fn mark(&mut self, val: u32) -> Option<(usize, usize)> {
        for (i, sq) in self.squares.iter_mut().enumerate() {
            if sq.value == val {
                sq.mark();
                return Some((i / NUM_COLS, i % NUM_COLS));
            }
        }
        return None;
    }

    fn score(&self, final_val: u32) -> u32 {
        let mut unmarked: u32 = 0;
        for sq in self.squares.into_iter() {
            if !sq.marked {
                unmarked += sq.value;
            }
        }
        unmarked * final_val
    }
    
    pub fn is_winning(&mut self) -> bool {
        for c in 0..NUM_COLS {
            let mut win: bool = true;
            for r in 0..NUM_ROWS {
                win &= self.sq(r, c).marked;
            }
            if win {
                return win;
            }
        }

        for r in 0..NUM_ROWS {
            let mut win: bool = true;
            for c in 0..NUM_COLS {
                win &= self.sq(r, c).marked;
            }
            if win {
                return win;
            }
        }

        false
    }

}

fn main() {
    let contents = fs::read_to_string(FILE).unwrap();
    let mut lines = contents.lines();

    // Pop the bingo draws first
    let draws = lines.next().unwrap(); 

    let mut boards: Vec<Board> = Vec::new(); 

    while lines.next().is_some() {
        let mut board = Board::new();
        for i in 0..5 {
            let row = lines.next().unwrap();
            for (j, num) in row.split_whitespace().enumerate() {
                board.sq(j, i).set_val(
                    num.parse().unwrap()
                );
            }
        }
        boards.push(board);
    }


    let mut wins: usize = boards.len();
    'draw: for (num, draw) in draws.split(",").enumerate() {
        for board in boards.iter_mut() {
            let draw_num: u32 = draw.parse().unwrap();
            board.mark(draw_num);
            if !board.finished && board.is_winning() {
                let score = board.score(draw_num);
                println!("win after {} draws with score {}", num+1, score);
                board.finished = true;
                wins -= 1;
            }
            if wins == 0 {
                println!("Done!");
                break 'draw;
            }
        }
    }
}
