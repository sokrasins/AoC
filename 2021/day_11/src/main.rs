use std::fs;
use std::fmt;

//const FILE: &str = "test.txt";
const FILE: &str = "final.txt";

const FLASH_THRESH: u32 = 9;

#[derive(Debug)]
struct Map {
    m: Vec<Octo>,
    max_rows: usize,
    max_cols: usize,
}

impl fmt::Display for Map {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.max_rows {
            for x in 0..self.max_cols {
                fmt.write_str(&self.idx(x as u32, y as u32).unwrap().value.to_string())?;
            }
            fmt.write_str("\n")?;
        }
        Ok(())
    }
}

#[derive(Debug)]
struct Octo {
    value: u32,
    has_flashed: bool,
}

impl Map {
    pub fn new() -> Self {
        Map {
            m: Vec::new(),
            max_rows: 0,
            max_cols: 0,
        }
    }

    pub fn add(&mut self, val: Octo) {
        self.m.push(val);
    }

    pub fn idx(&self, x: u32, y: u32) -> Option<&Octo> {
        // TODO: Error handling
        return Some(&self.m[(x as usize) + (y as usize) * self.max_cols]);
    }

    pub fn idx_mut(&mut self, x: u32, y: u32) -> Option<&mut Octo> {
        // TODO: Error handling
        return Some(&mut self.m[(x as usize) + (y as usize) * self.max_cols]);
    }

    fn incr(&mut self) {
        for x in 0..self.max_cols {
            for y in 0..self.max_rows {
                self.idx_mut(x as u32, y as u32).unwrap().value += 1;
            }
        }
    }

    fn flash(&mut self, x: u32, y: u32) {
        self.idx_mut(x, y).unwrap().has_flashed = true;

        //println!("flash x: {}, y: {}", x, y);

        let lower_x = if x == 0 { 0 } else { x - 1 };
        let lower_y = if y == 0 { 0 } else { y - 1 };
        let upper_x = if x as usize == (self.max_cols - 1) { x } else { x + 1 };
        let upper_y = if y as usize == (self.max_rows - 1) { y } else { y + 1 };

        //println!("lower_x: {}", lower_x);
        //println!("upper_x: {}", upper_x);
        //println!("lower_y: {}", lower_y);
        //println!("upper_y: {}", upper_y);

        for i_y in lower_y..(upper_y + 1) {
            for i_x in lower_x..(upper_x +1) {
                // TODO: Ignore this element
                self.idx_mut(i_x, i_y).unwrap().value += 1;
            }
        }

        for i_y in lower_y..(upper_y + 1) {
            for i_x in lower_x..(upper_x + 1) {
                if (i_x != x) || (i_y != y) {
                    //println!("x: {}, y: {}", i_x, i_y);
                    let octo = self.idx(i_x, i_y).unwrap();
                    if (octo.value > FLASH_THRESH) && (!octo.has_flashed) {
                        self.flash(i_x, i_y);
                    }
                }
            }
        }
    }

    pub fn step(&mut self) -> u32 {
        // 1. Increment
        self.incr();

        // 2. Flash
        for y in 0..self.max_rows {
            for x in 0..self.max_cols {
                let octo = self.idx(x as u32, y as u32).unwrap();
                if (octo.value > FLASH_THRESH) && (!octo.has_flashed) {
                    self.flash(x as u32, y as u32);
                }
            }
        }

        // 3. Reset
        let mut flash_count: u32 = 0;
        for octo in self.m.iter_mut() {
            if octo.has_flashed {
                flash_count += 1;
                octo.value = 0;
                octo.has_flashed = false;
            }
        }
        flash_count
    }
}

impl Octo {
    pub fn new(value: u32) -> Self {
        Octo {
            value,
            has_flashed: false,
        }
    }

    pub fn incr(&mut self) -> bool {
        let mut ret: bool = false;
        self.value += 1;
        if self.value > FLASH_THRESH {
            if !self.has_flashed {
                ret = true;
                self.has_flashed = true;
            }
        }
        ret
    }
}

fn main() {
    let contents = fs::read_to_string(FILE).unwrap();
    let lines = contents.lines();

    let mut map = Map::new();

    let mut row: u32 = 0;
    let mut col: u32 = 0;

    for line in lines {
        col = 0;

        if map.max_cols == 0 {
            map.max_cols = line.len();
        }

        for c in line.chars() {
            map.add(Octo::new(
                c.to_digit(10).unwrap()
            ));
            col += 1;
        }
        row += 1;
    }
    map.max_rows = row as usize;

    println!("{}", map);

    //let mut flashes: u32 = 0;
    //for _ in 0..100 {
    //    flashes += map.step();
    //}
    //
    //println!("total flashes: {}", flashes);
    let total = (map.max_rows * map.max_cols) as u32;

    for i in 0..1000 {
        if map.step() == total {
            println!("synced: {}", i+1);
        }
    }
}
