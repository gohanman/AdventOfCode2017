
extern crate proj_self;

struct Rule {
    from: String,
    to: String,
}
impl Rule {
    pub fn new(f: &str, t: &str) -> Rule {
        Rule { from: f.to_string(), to: t.to_string() }
    }
    
    pub fn from_string(s: &str) -> Rule {
        let parts: Vec<_> = s.split(" => ").collect();
        Rule { from: parts[0].to_string(), to: parts[1].to_string() }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Square {
    tiles: Vec<Vec<u8>>,
}
impl Square {
    pub fn default() -> Square {
        let tiles = vec![
            vec![0, 1, 0],
            vec![0, 0, 1],
            vec![1, 1, 1],
        ];

        Square { tiles: tiles }
    }

    pub fn from_rule(r: &str) -> Square {
        let rows: Vec<_> = r.split('/').collect();
        let mut tiles = vec![];
        for row in rows.iter() {
            let mut t = vec![];
            for c in row.chars() {
                match c {
                    '#' => t.push(1),
                    _ => t.push(0),
                }
            }
            tiles.push(t);
        }

        Square { tiles: tiles }
    }

    pub fn to_rule(&self) -> String {
        let mut rule = String::new();
        for y in 0..self.size() {
            for x in 0..self.size() {
                if self.tiles[y][x] == 0 {
                    rule.push('.');
                } else {
                    rule.push('#');
                }
            }
            if y < self.size() - 1 {
                rule.push('/');
            }
        }

        rule
    }

    pub fn from_squares(squares: &Vec<Square>) -> Square {
        let mut tiles = vec![];
        let per_row = (squares.len() as f64).sqrt() as usize;
        let mut idx = 0;
        /* size mismatches
        for (i, x) in squares.iter().enumerate() {
            println!("index {}, size {}", i, x.tiles.len());
        }
        */
        loop {
            for row_idx in 0..squares[idx].tiles.len() {
                let mut t = vec![];
                for sq_idx in 0..per_row {
                    let sq = &squares[idx + sq_idx];
                    for x in sq.tiles[row_idx].iter() {
                        t.push(*x);
                    }
                }
                tiles.push(t);
            }
            idx += per_row;
            if idx >= squares.len() {
                break;
            }
        }

        Square { tiles: tiles }
    }

    pub fn flip_v(&self) -> Square {
        let mut tiles = vec![];
        for y in (0..self.size()).rev() {
            let mut t = vec![];
            for x in 0..self.size() {
                t.push(self.tiles[y][x]);
            }
            tiles.push(t);
        }

        Square { tiles: tiles }
    }

    pub fn flip_h(&self) -> Square {
        let mut tiles = vec![];
        for y in 0..self.size() {
            let mut t = vec![];
            for x in (0..self.size()).rev() {
                t.push(self.tiles[y][x]);
            }
            tiles.push(t);
        }

        Square { tiles: tiles }
    }

    fn get_col(&self, x: usize) -> Vec<u8> {
        let mut ret = vec![];
        for y in 0..self.size() {
            ret.push(self.tiles[y][x]);
        }

        ret
    }

    pub fn rotate_l(&self) -> Square {
        let mut tiles = vec![];
        for x in (0..self.size()).rev() {
            let mut t = vec![];
            for val in self.get_col(x) {
                t.push(val);
            }
            tiles.push(t);
        }

        Square { tiles: tiles }
    }

    pub fn rotate_r(&self) -> Square {
        let mut tiles = vec![];
        for x in 0..self.size() {
            let mut t = vec![];
            for val in self.get_col(x).iter().rev() {
                t.push(*val);
            }
            tiles.push(t);
        }

        Square { tiles: tiles }

    }

    pub fn permute(&self) -> Vec<String> {
        let mut ret = vec![
            self.to_rule(),
            self.flip_v().to_rule(),
            self.flip_h().to_rule(),
        ];
        let mut rotator = self.clone();
        for _i in 0..4 {
            rotator = rotator.rotate_l();
            ret.push(rotator.to_rule());
            ret.push(rotator.flip_v().to_rule());
            ret.push(rotator.flip_h().to_rule());
        }

        ret
    }

    pub fn rule_applies(&self, rule: &Rule) -> bool {
        for str in self.permute() {
            if str == rule.from {
                return true;
            }
        }

        false
    }

    pub fn size(&self) -> usize {
        self.tiles[0].len()
    }

    pub fn slice(&self, x: usize, y: usize, size: usize) -> Square {
        let mut tiles = vec![];
        for i in y..(y+size) {
            let mut t = vec![];
            for j in x..(x+size) {
                t.push(self.tiles[i][j]);
            }
            tiles.push(t);
        }

        Square { tiles: tiles }
    }

    pub fn render(&self) {
        println!("Rendering:");
        for (y, _row) in self.tiles.iter().enumerate() {
            for t in self.tiles[y].iter() {
                print!("{}", if *t == 0 { '.' } else { '#' });
            }
            println!("");
        }
    }

    pub fn lights(&self) -> usize {
        let mut ret = 0;
        for (y, _row) in self.tiles.iter().enumerate() {
            for t in self.tiles[y].iter() {
                if *t == 1 {
                    ret += 1;
                }
            }
        }

        ret
    }
}

fn grow(sq: &Square, rules: &Vec<Rule>) -> Square {
    let mut squares = vec![];
    let slice_size = if sq.size() % 2 == 0 { 2 } else { 3 };
    let mut y = 0;
    loop {
        let mut x = 0;
        loop {
            squares.push(sq.slice(x, y, slice_size));
            x += slice_size;
            if x >= sq.size() {
                break;
            }
        }
        y += slice_size;
        if y >= sq.size() {
            break;
        }
    }
    for i in 0..squares.len() {
        let permutations = squares[i].permute();
        for rule in rules.iter() {
            for p in permutations.iter() {
                if *p == rule.from {
                    squares[i] = Square::from_rule(&rule.to);
                    break;
                }
            }
        }
    }

    if squares.len() == 1 {
        squares[0].clone()
    } else {
        Square::from_squares(&squares)
    }
}

fn main() {
    let proj = proj_self::proj_dir(3);
    let file = proj.join("input.txt");
    let input = proj_self::file_to_str(&file);
    let lines = proj_self::str_to_lines(&input);
    let rules: Vec<Rule> = lines.iter().map(|x| Rule::from_string(x)).collect();
    let mut sq = Square::default();
    for _i in 0..5 {
        sq = grow(&sq, &rules);
        sq.render();
    }
    println!("-----------------------------");
    println!("Lights: {}", sq.lights());
    println!("-----------------------------");

    sq = Square::default();
    for _i in 0..18 {
        sq = grow(&sq, &rules);
        //sq.render();
        println!("Pass: {}, Size: {}", _i, sq.size());
    }
    println!("-----------------------------");
    println!("Lights: {}", sq.lights());
    println!("-----------------------------");
}

#[test]
fn test() {
    let sq = Square::default();
    sq.render();
    let new_sq = Square::from_squares(&vec![sq.clone(), sq.clone(), sq.clone(), sq.clone()]);
    new_sq.render();
    let rules = vec![
        Rule::new("../.#", "##./#../..."),
        Rule::new(".#./..#/###", "#..#/..../..../#..#"),
    ];
    let mut growing = sq.clone();
    for _i in 0..2 {
        growing = grow(&growing, &rules);
        growing.render();
    }
}
