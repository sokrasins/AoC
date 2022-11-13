use std::fs;
use std::cmp::max;
//use std::vec::Vec;

//const FILE: &str = "test.txt";
const FILE: &str = "final.txt";

#[derive(Debug, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

struct LineSeg {
    done: bool,
    p1: Point,
    p2: Point,
}

struct Map {
    m: Vec<u32>,
    max_x: usize,
    max_y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Point { x, y }
    }

    pub fn max(p1: &Self, p2: &Self) -> Self {
        Point {
            x: max(p1.x, p2.x), 
            y: max(p1.y, p2.y),
        }
    }
}

impl LineSeg {
    pub fn new(p1: Point, p2: Point) -> Self {
        LineSeg { p1, p2, done: false }
    }
}

impl Iterator for LineSeg {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {

        if self.p1.x == self.p2.x && self.p1.y == self.p2.y {
            if self.done {
                return None;
            }
            self.done = true;
        }

        let point = self.p1;

        let next_x = if self.p1.x < self.p2.x {
            self.p1.x + 1
        } else if self.p1.x > self.p2.x {
            self.p1.x - 1
        } else {
            self.p1.x
        };

        let next_y = if self.p1.y < self.p2.y {
            self.p1.y + 1
        } else if self.p1.y > self.p2.y {
            self.p1.y - 1
        } else {
            self.p1.y
        };

        self.p1 = Point::new(next_x, next_y);

        Some(point)
    }
}

impl Map {
    pub fn new(max_x: usize, max_y: usize, i: u32) -> Self {
        let mut m: Vec<u32> = Vec::new();
        m.resize(max_x * max_y, i);

        Map { 
            m,
            max_x,
            max_y,
        }
    }

    pub fn idx(&mut self, x: usize, y: usize) -> &mut u32 {
        &mut self.m[x+(y*self.max_y)]
    }

    pub fn draw(&mut self, seg: &mut LineSeg) {
        for p in seg {
            *self.idx(p.x, p.y) +=  1;
        }
    }
    
    pub fn multimarked(&self) -> u32 {
        let mut tot: u32 = 0;
        for x in self.m.iter() {
            if *x > 1 {
                tot += 1;
            }
        }
        tot
    }
}

fn parse_line(line: &str) -> (Point, Point) {
    // Split the line into: p1_repr, '->', p2_repr
    let mut points = line.split_whitespace();

    // Pll out the point reprs, throw away the arrow 
    let mut p1_repr_iter = points.next().unwrap().split(",");
    _ = points.next();
    let mut p2_repr_iter = points.next().unwrap().split(",");

    let p1 = Point::new(
        p1_repr_iter.next().unwrap().parse::<usize>().unwrap(),
        p1_repr_iter.next().unwrap().parse::<usize>().unwrap(),
    );

    let p2 = Point::new(
        p2_repr_iter.next().unwrap().parse::<usize>().unwrap(),
        p2_repr_iter.next().unwrap().parse::<usize>().unwrap(),
    );
       
    (p1, p2)
}

fn main() {
    let contents = fs::read_to_string(FILE).unwrap();
    let lines = contents.lines();

    let mut line_segs: Vec<LineSeg> = Vec::new();
    let mut boundary: Point = Point::new(0, 0);

    for line in lines {
        let (p1, p2) = parse_line(&line);
        boundary = Point::max(&boundary, &p1);
        boundary = Point::max(&boundary, &p2);
        line_segs.push(LineSeg::new(p1, p2));
    }

    println!("boundary point: {:?}", boundary);

    let mut map = Map::new((boundary.x + 1) as usize, (boundary.y + 1) as usize, 0 as u32);

    for mut line in line_segs {
        map.draw(&mut line);
    }

    let val = map.multimarked();

    println!("{}", val);

}
