
extern crate proj_self;

enum Move {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
    Invalid,
}

type Programs = Vec<char>;

fn init_programs(size: usize) -> Programs {
    let mut ret = vec!['a'; size];
    for i in 1..size {
        let ascii = (97 + i) as u8;
        ret[i] = ascii as char;
    }

    ret
}

fn str_to_move(s: &str) -> Move {
    let mut rest = s.trim().to_string();
    let first = rest.remove(0);
    match first {
        's' => Move::Spin(rest.parse().unwrap()),
        'x' => {
            let parts: Vec<_> = rest.split("/").collect();
            Move::Exchange(parts[0].parse().unwrap(), parts[1].parse().unwrap())
        },
        'p' => {
            let parts: Vec<char> = rest.chars().collect();
            Move::Partner(parts[0], parts[2])
        }
        _ => Move::Invalid,
    }
}

fn apply_move(p: &Programs, m: &Move) -> Programs {
    match *m {
        Move::Spin(x) => spin(p, x),
        Move::Exchange(x, y) => exchange(p, x, y),
        Move::Partner(x, y) => partner(p, x, y),
        Move::Invalid => p.clone(),
    }
}

fn spin(p: &Programs, x: usize) -> Programs {
    let mut ret = vec![];
    for i in 0..p.len() {
        let pos = ((p.len() - x) + i) % p.len();
        ret.push(p[pos]);
    }

    ret
}

fn exchange(p: &Programs, x: usize, y: usize) -> Programs {
    let mut ret = vec![];
    for i in 0..p.len() {
        if i == x {
            ret.push(p[y]);
        } else if i == y {
            ret.push(p[x]);
        } else {
            ret.push(p[i]);
        }
    }

    ret
}

fn partner(p: &Programs, x: char, y: char) -> Programs {
    let mut ret = p.clone();
    for i in 0..ret.len() {
        if ret[i] == x {
            ret[i] = y;
        } else if ret[i] == y {
            ret[i] = x;
        }
    }

    ret
}

fn dance(p: &Programs, m: &Vec<Move>) -> Programs {
    m.iter().fold(p.clone(), | acc, x | apply_move(&acc, x))
}

fn as_str(chars: &Vec<char>) -> String {
    chars.iter().cloned().collect()
}

fn main() {
    let proj = proj_self::proj_dir(3);
    let file = proj.join("input.txt");
    let input = proj_self::file_to_str(&file);
    let moves: Vec<Move> = input.split(",").map(|x| str_to_move(x)).collect();
    let init = init_programs(16);
    let end = dance(&init, &moves);
    println!("Ending: {}", as_str(&end));
    let mut many = init_programs(16);
    let mut map = std::collections::HashMap::new();
    let mut cycle = 0;
    for i in 0..1000000000 {
        eprint!("{}\r", i);
        if map.contains_key(&many) {
            println!("Cycled at {}", i);
            cycle = i;
            break;
        }
        map.insert(many.clone(), true);
        many = dance(&many, &moves);
    }
    let mut next = cycle;
    while next + cycle < 1000000000 {
        next += cycle;
    }
    println!("Resume at {}", next);
    for _i in next..1000000000 {
        many = dance(&many, &moves);
    }
    println!("And then: {}", as_str(&many));
}

#[test]
fn test() {
    assert_eq!(vec!['a', 'b', 'c'], init_programs(3));
    let init = init_programs(5);
    let inputs = vec!["s1", "x3/4", "pe/b"];
    let moves: Vec<Move> = inputs.iter().map(|x| str_to_move(x)).collect();
    let end = dance(&init, &moves);
    assert_eq!("baedc", as_str(&end));
    let next = dance(&end, &moves);
    assert_eq!("ceadb", as_str(&next));
}
