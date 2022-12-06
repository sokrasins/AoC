use std::fs;

//const FILE: &str = "test.txt";
//const FILE: &str = "test2.txt";
//const FILE: &str = "test3.txt";
const FILE: &str = "final.txt";


#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Size {
    Tiny,       // For 'start' and 'end'
    Small,
    Big,
}

#[derive(Debug, Clone)]
struct Room {
    name: String,
    size: Size,
    connections: Vec<String>,
}

#[derive(Debug, Clone)]
struct Cave {
    rm: Vec<Room>,
}

impl Room {
    fn new(name: String) -> Self {
        let size = match name.as_str() {
            "start" | "end" => Size::Tiny,
            _ => if name.chars().all( |c| c.is_uppercase() ) {
                Size::Big 
            } else {
                Size::Small 
            }
        };

        Room {
            name,
            size,
            connections: Vec::new(),
        }
    }
}

impl Cave {
    fn new() -> Self {
        Cave { rm: Vec::new() }
    }

    fn add_room(&mut self, room: Room) {
        self.rm.push(room);
    }

    fn find_room_idx(&self, name: &str) -> Option<usize> {
        for (i, r) in self.rm.iter().enumerate() {
            if r.name.eq(name) {
                return Some(i);
            }
        }
        None
    }

    fn room(&self, name: &str) -> Option<&Room> {
        match self.find_room_idx(name) {
            Some(i) => Some(&self.rm[i]),
            None => None,
        }
    }

    fn room_size(&self, name: &str) -> Option<Size> {
        match self.find_room_idx(name) {
            Some(i) => Some(self.rm[i].size), 
            None => None,
        }
    }

    fn exists(&self, name: &str) -> bool {
        match self.find_room_idx(name) {
            Some(_) => true,
            None => false,
        }
    }

    fn add_connection(&mut self, room1: &str, room2: &str) {
        let room1_idx = self.find_room_idx(&room1).unwrap();
        let room2_idx = self.find_room_idx(&room2).unwrap();
        
        self.rm[room1_idx].connections.push(room2.to_string()); 
        self.rm[room2_idx].connections.push(room1.to_string()); 

        self.rm[room1_idx].connections.dedup();
        self.rm[room2_idx].connections.dedup();
    }

    fn connections(&self, room: &String) -> &Vec<String> {
        let idx = self.find_room_idx(room).unwrap();
        return &self.rm[idx].connections;
    }
}

#[derive(Debug, Clone)]
struct Path {
    seq: Vec<String>,
    //visited: Vec<String>, 
}

impl Path {
    fn new() -> Self {
        Path { seq: vec!["start".to_string()] }
    }

    fn valid_step(&self, room: &str, cave: &Cave) -> bool {
        let rm = cave.room(room).unwrap();
        let num_visits = self.visits(room);

        let mut visited_list = self.seq.clone();
        visited_list.dedup();

        let max_small_visits: usize = visited_list.iter().map(|v_loc| {
            if cave.room_size(v_loc).unwrap() == Size::Small {
                return self.visits(v_loc)
            } else {
                return 0 as usize;
            }
        }).max().unwrap();

        return match rm.size {
            Size::Tiny => num_visits < 1,
            Size::Small => {
                if max_small_visits >= 2 {
                    return num_visits < 1;
                } else {
                    return num_visits < 2;
                }
            },
            Size::Big => true,
        }
    }

    fn visits(&self, room: &str) -> usize {
        return self
            .seq
            .iter()
            .filter(|visited_room| visited_room.as_str().eq(room))
            .count();   
    }

    fn step(&mut self, room: &str) {
        self.seq.push(room.to_string()); 
    }
}

fn find_paths(p: Path, c: &Cave) -> Vec<Path> {
    let last_room = p.seq.last().unwrap();
    if last_room.eq("end") {
        return vec![p];
    } else {
        let next_rooms = c.connections(last_room); 

        let mut ret: Vec<Path> = Vec::new();

        for room in next_rooms.iter() {
            if p.valid_step(room, c) {
                let mut new_path = p.clone();
                new_path.step(room);
                ret.append(&mut find_paths(new_path, c))
            }
        }
        return ret
    }    
}

fn main() {
    let contents = fs::read_to_string(FILE).unwrap();
    let lines = contents.lines();

    let mut cave = Cave::new();
    
    for line in lines {
        println!("{}", line);

        let mut rooms = line.split("-");
        let room1 = rooms.next().unwrap();
        let room2 = rooms.next().unwrap();

        if !cave.exists(room1) {
            cave.add_room(Room::new(String::from(room1)));
        }

        if !cave.exists(room2) {
            cave.add_room(Room::new(String::from(room2)));
        }

        cave.add_connection(room1, room2);
    }

    let all_paths = find_paths(Path::new(), &cave);
    println!("{:?}", all_paths);
    println!("{}", all_paths.len());
}
