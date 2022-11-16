use std::fs;

//const FILE: &str = "test.txt";
const FILE: &str = "final.txt";

struct Loc {
    height: u32,
    min: bool,
    size: u32,
}

impl Loc {
    fn new(height: u32) -> Loc {
        Loc { height, min: false, size: 0, }
    }
}

struct Map {
    locs: Vec<Loc>,
    x: usize,
    y: usize,
}

impl Map {
    pub fn new() -> Map {
        Map {
            locs: Vec::new(),
            x: 0,
            y: 0,
        }
    }

    pub fn idx(&self, x: i32, y: i32) -> Option<&Loc> {
        if x >= self.x as i32 || y >= self.y as i32 || x < 0 || y < 0 {
            return None;
        }
        Some( &self.locs[x as usize + (y as usize)*self.x] )
    }

    pub fn idx_mut(&mut self, x: i32, y: i32) -> Option<&mut Loc> {
        if x >= self.x as i32 || y >= self.y as i32 || x < 0 || y < 0 {
            return None;
        }
        Some( &mut self.locs[x as usize + (y as usize)*self.x] )
    }


    pub fn lowest_neighbor(&self, x: i32, y: i32) -> Option<(i32, i32)> {
        let neighbors: [(i32, i32); 5] = [
            (x, y), 
            (x-1, y), 
            (x+1, y), 
            (x, y-1), 
            (x, y+1) 
        ];

        // Get heights of all neighbors
        let heights: Vec<Option<u32>> = neighbors.iter().map(|pt| {
            if let Some(loc) = self.idx(pt.0, pt.1) {
                return Some(loc.height);
            }
            return None;
        }).collect();

        // find min idx
        let mut min_idx: usize = 0;
        for (i, height) in heights.iter().enumerate() {
            min_idx = match height {
                Some(h) => if h < &heights[min_idx].unwrap() { i } else { min_idx },
                None => min_idx,
            }
        }

        return match min_idx {
            0 => None,
            _ => Some(neighbors[min_idx]),
        };
    }

    pub fn is_lower(&self, x: i32, y: i32) -> bool {
        let neighbors: [(i32, i32); 5] = [
            (x, y), 
            (x-1, y), 
            (x+1, y), 
            (x, y-1), 
            (x, y+1) 
        ];

        // Get heights of all neighbors
        let heights: Vec<Option<u32>> = neighbors.iter().map(|pt| {
            if let Some(loc) = self.idx(pt.0, pt.1) {
                return Some(loc.height);
            }
            return None;
        }).collect();

        !heights.iter().all(|&x| if x.is_some() { x.unwrap() == heights[0].unwrap() } else { true })

    }

    pub fn points(&self) -> u32 {
        let mut score: u32 = 0;
        for loc in self.locs.iter() {
            if loc.min {
                score += loc.height + 1;
                println!("size: {:?}", loc.size);
            }
        }
        score
    }

    pub fn biggest(&self) -> (u32, u32, u32) {
        let mut size: Vec<u32> = self.locs.iter().map(|x| x.size).collect();
        size.sort();

        (
            size[size.len() - 1], 
            size[size.len() - 2], 
            size[size.len() - 3],
        )
    }
}

fn main() {
    let contents = fs::read_to_string(FILE).unwrap();
    let lines = contents.lines();

    let mut map = Map::new();
    
    for line in lines {
        if map.x == 0 {
            map.x = line.len();
        }

        for c in line.chars() {
            map.locs.push(Loc::new(c.to_digit(10).unwrap()));            
        }
    }
    map.y = (map.locs.len()) / map.x;

    println!("({}, {})", map.x, map.y);

    for y in 0..map.y {
        for x in 0..map.x {
            let mut low: Option<(i32, i32)> = Some((x as i32, y as i32));
            let orig_height = map.idx(x as i32, y as i32).unwrap().height;
            while low.is_some() {
                low = match map.lowest_neighbor(low.unwrap().0, low.unwrap().1) {
                    None => {
                        // Exclude points that are at equal height to all neighbors
                        if map.is_lower(low.unwrap().0, low.unwrap().1) {
                            let mut loc = map.idx_mut(low.unwrap().0, low.unwrap().1).unwrap();
                            loc.min = true;
                            if orig_height != 9 {
                                loc.size += 1;
                            }
                        } 
                        None
                    },
                    x => x,
                }
            }
        }
    }
    
    let pts = map.points();
    let (v1, v2, v3) = map.biggest();
    let size_score = v1 * v2 * v3;

    println!("{}", pts);
    println!("{:?}", size_score);
}
