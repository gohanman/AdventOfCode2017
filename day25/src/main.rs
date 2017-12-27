
extern crate proj_self;

#[derive(Debug, Eq, PartialEq, Clone)]
enum Dir { L, R }

#[derive(Debug, Eq, PartialEq, Clone)]
struct Step {
    write: u8,
    shift: Dir,
    next: char,
}
impl Step {
    pub fn new(w: u8, s: Dir, n: char) -> Step {
        Step { write: w, shift: s, next: n }
    }

    pub fn factory(lines: &Vec<&str>) -> (char, u64, std::collections::HashMap<char,(Step,Step)>) {
        let mut steps = std::collections::HashMap::new();
        let start_parts: Vec<_> = lines[0].split(' ').map(|x| x.trim_matches('.')).collect();
        let start: char = start_parts.last().unwrap().chars().nth(0).unwrap();
        let checksum_parts: Vec<_> = lines[1].split(' ').collect();
        let checksum: u64 = checksum_parts[5].parse().unwrap();

        let mut cur = 2;
        loop {
            let st_pts: Vec<_> = lines[cur].split(' ').map(|x| x.trim_matches(':')).collect();
            if st_pts.len() != 3 {
                break;
            }
            let state: char = st_pts[2].chars().nth(0).unwrap();
            
            let w0_pts: Vec<_> = lines[cur+2].split(' ').map(|x| x.trim_matches('.')).collect();
            let w0: u8 = w0_pts.last().unwrap().parse().unwrap();
            let s0_pts: Vec<_> = lines[cur+3].split(' ').map(|x| x.trim_matches('.')).collect();
            let s0: Dir = match *s0_pts.last().unwrap() {
                "right" => Dir::R,
                _ => Dir::L,
            };
            let n0_pts: Vec<_> = lines[cur+4].split(' ').map(|x| x.trim_matches('.')).collect();
            let n0: char = n0_pts.last().unwrap().chars().nth(0).unwrap();

            let w1_pts: Vec<_> = lines[cur+6].split(' ').map(|x| x.trim_matches('.')).collect();
            let w1: u8 = w1_pts.last().unwrap().parse().unwrap();
            let s1_pts: Vec<_> = lines[cur+7].split(' ').map(|x| x.trim_matches('.')).collect();
            let s1: Dir = match *s1_pts.last().unwrap() {
                "right" => Dir::R,
                _ => Dir::L,
            };
            let n1_pts: Vec<_> = lines[cur+8].split(' ').map(|x| x.trim_matches('.')).collect();
            let n1: char = n1_pts.last().unwrap().chars().nth(0).unwrap();

            steps.insert(state, (Step::new(w0, s0, n0), Step::new(w1, s1, n1)));
            cur += 9;

            if cur >= lines.len() {
                break;
            }
        }
    
        (start, checksum, steps)
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Turing {
    tape: std::collections::HashMap<i32, u8>,
    cursor: i32,
    state: char,
    states: std::collections::HashMap<char, (Step, Step)>,
}
impl Turing {
    pub fn new(init: &char, s: &std::collections::HashMap<char, (Step,Step)>) -> Turing {
        let t = std::collections::HashMap::new();
        Turing { tape: t, cursor: 0, state: *init, states: s.clone() }
    }

    pub fn read(&self) -> u8 {
        self.get(&self.cursor)
    }

    pub fn get(&self, i: &i32) -> u8 {
        match self.tape.get(&i) {
            Some(x) => *x,
            None => 0,
        }
    }

    pub fn step(&mut self) {
        let cur = self.read();
        let both = self.states.get(&self.state).unwrap().clone();
        let active = if cur == 0 { both.0 } else { both.1 }; 
        self.tape.insert(self.cursor, active.write);
        self.cursor += match active.shift {
            Dir::L => -1,
            Dir::R => 1, 
        };
        self.state = active.next;
    }

    pub fn checksum(&self) -> u64 {
        let mut ttl = 0;
        for (_state, val) in &self.tape {
            ttl += (*val as u64);
        }

        ttl
    }

    pub fn print(&self) {
        for i in -10..11 {
            print!("{} ", self.get(&i));
        }
        println!("");
        for i in -10..11 {
            if i == self.cursor {
                print!("^ ");
            } else {
                print!("  ");
            }
        }
        println!("\n");
    }
}


fn main() {
    let proj = proj_self::proj_dir(3);
    let file = proj.join("input.txt");
    let input = proj_self::file_to_str(&file);
    let lines = proj_self::str_to_lines(&input);

    let (init, after, states) = Step::factory(&lines);
    let mut turing = Turing::new(&init, &states);
    for i in 0..after {
        if i % 5000 == 0 {
            eprint!("{}\r", i);
        }
        turing.step();
    }
    println!("Checksum: {}", turing.checksum());
}

#[test]
fn test() {
    let lines = vec![
        "Begin in state A.",
        "Perform a diagnostic checksum after 6 steps.",
        "In state A:",
        "   If the current value is 0:",
        "       - Write the value 1.",
        "       - Move one slot to the right.",
        "       - Continue with state B.",
        "   If the current value is 1:",
        "       - Write the value 0.",
        "       - Move one slot to the left.",
        "       - Continue with state B.",
        "In state B:",
        "   If the current value is 0:",
        "       - Write the value 1.",
        "       - Move one slot to the left.",
        "       - Continue with state A.",
        "   If the current value is 1:",
        "       - Write the value 1.",
        "       - Move one slot to the right.",
        "       - Continue with state A.",
    ];
    let (init, after, states) = Step::factory(&lines);
    let mut turing = Turing::new(&init, &states);
    for _i in 0..after {
        turing.print();
        turing.step();
    }
    turing.print();
    assert_eq!(turing.checksum(), 3);
}
