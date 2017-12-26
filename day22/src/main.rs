
extern crate proj_self;

#[derive(Debug, Clone, Eq, PartialEq)]
enum Status {
    Clean,
    Infected,
    Weak,
    Flagged,
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Direction {
    U,
    D,
    L,
    R,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Carrier {
    x: i32,
    y: i32,
    dir: Direction,
    tally: usize,
}
impl Carrier {
    pub fn new(x: i32, y: i32) -> Carrier {
        Carrier { x: x, y: y, dir: Direction::U, tally: 0 }
    }

    pub fn turn_left(&mut self) {
        self.dir = match self.dir {
            Direction::U => Direction::L,
            Direction::D => Direction::R,
            Direction::L => Direction::D,
            Direction::R => Direction::U,
        };
    }

    pub fn turn_right(&mut self) {
        self.dir = match self.dir {
            Direction::U => Direction::R,
            Direction::D => Direction::L,
            Direction::L => Direction::U,
            Direction::R => Direction::D,
        };
    }

    pub fn reverse(&mut self) {
        self.dir = match self.dir {
            Direction::U => Direction::D,
            Direction::D => Direction::U,
            Direction::L => Direction::R,
            Direction::R => Direction::L,
        };
    }

    pub fn move_one(&mut self) {
        match self.dir {
            Direction::U => self.y -= 1,
            Direction::D => self.y += 1,
            Direction::L => self.x -= 1,
            Direction::R => self.x += 1,
        }
    }

    pub fn inc(&mut self) {
        self.tally += 1;
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Tiles {
    tiles: std::collections::HashMap<(i32,i32),Status>,
}
impl Tiles {
    pub fn new(lines: &Vec<&str>) -> Tiles {
        let mut tiles = std::collections::HashMap::new();
        let mut y = 0;
        for line in lines.iter() {
            let mut x = 0;
            for c in line.chars() {
                let ins = match c {
                    '#' => Status::Infected,
                    _ => Status::Clean,
                };
                tiles.insert((x,y), ins);
                x += 1;
            }
            y += 1;
        }

        Tiles { tiles: tiles }
    }

    pub fn is_infected(&self, x: i32, y: i32) -> bool {
        match self.tiles.get(&(x,y)) {
            Some(i) => if *i == Status::Infected { true } else { false },
            None => false,
        }
    }

    pub fn infect(&mut self, x: i32, y: i32) {
        self.tiles.insert((x,y), Status::Infected);
    }

    pub fn clean(&mut self, x: i32, y: i32) {
        self.tiles.insert((x,y), Status::Clean);
    }

    pub fn get_status(&self, x: i32, y: i32) -> Status {
        match self.tiles.get(&(x,y)) {
            Some(i) => i.clone(),
            None => Status::Clean,
        }
    }

    pub fn rotate_status(&mut self, x: i32, y: i32) {
        let next = match self.get_status(x, y) {
            Status::Clean => Status::Weak,
            Status::Weak => Status::Infected,
            Status::Infected => Status::Flagged,
            Status::Flagged => Status::Clean,
        };
        self.tiles.insert((x,y), next);
    }
}

fn bursts(t: &Tiles, c: &Carrier, i: usize) -> Carrier {
    let mut tiles = t.clone();
    let mut carrier = c.clone();
    for _i in 0..i {
        if tiles.is_infected(carrier.x, carrier.y) {
            tiles.clean(carrier.x, carrier.y);
            carrier.turn_right();
        } else {
            tiles.infect(carrier.x, carrier.y);
            carrier.turn_left();
            carrier.inc();
        }
        carrier.move_one();
    }

    carrier
}

fn bursts2(t: &Tiles, c: &Carrier, i: usize) -> Carrier {
    let mut tiles = t.clone();
    let mut carrier = c.clone();
    for _i in 0..i {
        match tiles.get_status(carrier.x, carrier.y) {
            Status::Clean => carrier.turn_left(),
            Status::Infected => carrier.turn_right(),
            Status::Weak => carrier.inc(),
            Status::Flagged => carrier.reverse(),
        };
        tiles.rotate_status(carrier.x, carrier.y);
        carrier.move_one();
        if _i % 1000 == 0 {
            eprint!("{}\t{}\r", carrier.tally, _i);
        }
    }

    carrier
}


fn main() {
    let proj = proj_self::proj_dir(3);
    let file = proj.join("input.txt");
    let input = proj_self::file_to_str(&file);
    let lines = proj_self::str_to_lines(&input);

    let tiles = Tiles::new(&lines);
    let carrier = Carrier::new(12, 12);
    let after = bursts(&tiles, &carrier, 10000);
    println!("After 10000: {}", after.tally);

    let after2 = bursts2(&tiles, &carrier, 10000000);
    println!("After 10000000: {}", after2.tally);
}

#[test]
fn test() {
    let lines = vec![
        "..#",
        "#..",
        "...",
    ];
    let tiles = Tiles::new(&lines);
    let carrier = Carrier::new(1, 1);
    let after7 = bursts(&tiles, &carrier, 7);
    assert_eq!(after7.tally, 5);
    let after70 = bursts(&tiles, &carrier, 70);
    assert_eq!(after70.tally, 41);

    let after100 = bursts2(&tiles, &carrier, 100);
    assert_eq!(after100.tally, 26);
}
