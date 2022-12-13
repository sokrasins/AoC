use std::fs;

//const FILE: &str = "test.txt";
const FILE: &str = "final.txt";

#[derive(Clone, Debug)]
struct Grid<T: Clone + Copy> {
    rows: usize,
    cols: usize,
    items: Vec<Option<T>>,
}

impl<T: Clone + Copy> Grid<T> {
    fn new(rows: usize, cols: usize) -> Self {
        let mut items = Vec::new();
        items.resize(rows * cols, None);

        Self {
            rows,
            cols,
            items, 
        }
    }

    fn to_idx(&self, r: usize, c: usize) -> Option<usize> {
        if r >= self.rows || c >= self.cols {
            return None;
        } else {
            return Some(c + r * (self.cols))
        }
    }

    fn get(&self, r: usize, c: usize) -> &Option<T> {
        let idx = self.to_idx(r, c).unwrap(); 
        &self.items[idx]
    }

    fn val(&self, r: usize, c: usize) -> T {
        let idx = self.to_idx(r, c).unwrap(); 
        self.items[idx].unwrap()
    }

    fn set(&mut self, r: usize, c: usize, item: T) {
        let idx = self.to_idx(r, c).unwrap(); 
        self.items[idx] = Some(item);
    }

    fn clear(&mut self, r: usize, c: usize) {
        let idx = self.to_idx(r, c).unwrap(); 
        self.items[idx] = None;
    }

    fn is_set(&self, r: usize, c: usize) -> bool {
        let idx = self.to_idx(r, c).unwrap(); 
        self.items[idx].is_some()
    }
}

impl Grid<Tree> {
    fn visible(&self, r: usize, c: usize) -> bool {
        let cur_height = self.val(r, c).height;
        let mut visible = false;

        // Top
        visible |= (0..r).all(|x| self.val(x, c).height < cur_height);
        // Bottom
        visible |= ((r+1)..self.rows).all(|x| self.val(x, c).height < cur_height);
        // Left
        visible |= (0..c).all(|x| self.val(r, x).height < cur_height);
        //Right
        visible |= ((c+1)..self.cols).all(|x| self.val(r, x).height < cur_height);

        visible
    }

    fn view(&self, r: usize, c: usize) -> u32 {
        let cur_height = self.val(r, c).height;
        let mut score = 1;

        let mut num_visible = 0;
        for i in (0..r).rev() {
            num_visible += 1;
            if self.val(i, c).height >= cur_height {
                break;
            }
        }
        score *= num_visible;

        num_visible = 0;
        for i in (r+1)..self.rows {
            num_visible += 1;
            if self.val(i, c).height >= cur_height {
                break;
            }
        }
        score *= num_visible;

        num_visible = 0;
        for i in (0..c).rev() {
            num_visible += 1;
            if self.val(r, i).height >= cur_height {
                break;
            }
        }
        score *= num_visible;

        num_visible = 0;
        for i in (c+1)..self.cols {
            num_visible += 1;
            if self.val(r, i).height >= cur_height {
                break;
            }
        }
        score *= num_visible;

        score
    }
}

#[derive(Clone, Copy, Debug)]
struct Tree {
    height: u32,
}

impl Tree {
    fn new(height: u32) -> Self {
        Self {
            height,
        }
    }
}

fn main() {
    let contents = fs::read_to_string(FILE).unwrap();
    let lines = contents.lines();

    let rows = contents.lines().count();
    let cols = contents.lines().peekable().peek().unwrap().len();

    let mut g = Grid::<Tree>::new(rows, cols);

    for (j, line) in lines.enumerate() {
        println!("{}", line);

        for (i, c) in line.chars().enumerate() {
            g.set(j, i, Tree::new(c.to_digit(10).unwrap()));
        }
    }

    let mut num_vis = 0;
    for i in 0..g.rows {
        for j in 0..g.cols {
            num_vis += if g.visible(i, j) { 1 } else { 0 };
        }
    }

    let mut max_score = 0;
    for i in 0..g.rows {
        for j in 0..g.cols {
            let score = g.view(i, j);
            if score > max_score {
                max_score = score;
            }
        }
    }

    //println!("{:?}", g);
    println!("num vis: {}", num_vis);
    println!("max score: {}", max_score);
}
