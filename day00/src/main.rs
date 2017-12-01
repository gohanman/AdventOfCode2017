
use std::path::PathBuf;
use std::io::Read;
use std::vec::Vec;

enum KeyPress {
    Up,
    Down,
    Left,
    Right,
    A,
    B,
    Start,
    Unknown,
}

#[derive(Clone)]
enum Mark {
    A,
    B,
    None,
}

#[derive(Clone)]
struct Point {
    x: i32,
    y: i32,
    mark: Mark,
}

fn proj_dir() -> PathBuf {

    fn rm_dirs(num: u32, path: PathBuf) -> PathBuf {
        if num == 0 {
            return path;
        }

        let parent = path.parent().unwrap().to_path_buf();
        rm_dirs(num - 1, parent)
    }

    let exe = std::env::current_exe().unwrap();
    rm_dirs(3, exe)
}

fn walk(cur: Point, key: &KeyPress) -> Point {
    match *key {
        KeyPress::Up => Point {
            x: cur.x,
            y: (cur.y + 1),
            mark: Mark::None,
        },
        KeyPress::Down => Point {
            x: cur.x,
            y: (cur.y - 1),
            mark: Mark::None,
        },
        KeyPress::Left => Point {
            x: (cur.x - 1),
            y: cur.y,
            mark: Mark::None,
        },
        KeyPress::Right => Point {
            x: (cur.x + 1),
            y: cur.y,
            mark: Mark::None,
        },
        KeyPress::A => Point {
            mark: Mark::A,
            ..cur
        },
        KeyPress::B => Point {
            mark: Mark::B,
            ..cur
        },
        _ => cur,
    }
}

fn str_to_key(input: &str) -> KeyPress {
    match input {
        "Up" => KeyPress::Up,
        "Down" => KeyPress::Down,
        "Left" => KeyPress::Left,
        "Right" => KeyPress::Right,
        "A" => KeyPress::A,
        "B" => KeyPress::B,
        "Start" => KeyPress::Start,
        _ => KeyPress::Unknown,
    }
}

fn to_key_presses(input: &str) -> Vec<KeyPress> {
    input
        .split(',')
        .map(|x| x.trim())
        .map(|x| str_to_key(x))
        .collect()
}

fn get_points(input: &str) -> Vec<Point> {
    let keys = to_key_presses(&input);
    let cur = Point {
        x: 0,
        y: 0,
        mark: Mark::None,
    };
    let mut points: Vec<Point> = Vec::new();
    let _end = keys.iter().fold(cur, |acc, x| {
        points.push(acc.clone());
        walk(acc, x)
    });
    points
}

fn distance(p1: &Point, p2: &Point) -> i32 {
    let xdiff = p1.x - p2.x;
    let ydiff = p1.y - p2.y;
    xdiff.abs() + ydiff.abs()
}

fn furthest(points: &Vec<Point>) -> i32 {
    let origin = Point {
        x: 0,
        y: 0,
        mark: Mark::None,
    };
    let mut marked: Vec<i32> = points
        .iter()
        .filter(|x| match x.mark {
            Mark::None => false,
            _ => true,
        })
        .map(|x| distance(&origin, x))
        .collect();
    marked.sort();
    match marked.pop() {
        Some(x) => x,
        None => 0,
    }
}

fn is_a(p: &Point) -> bool {
    match p.mark {
        Mark::A => true,
        _ => false,
    }
}

fn is_b(p: &Point) -> bool {
    match p.mark {
        Mark::B => true,
        _ => false,
    }
}

fn pair(points: &Vec<Point>) -> i32 {
    let a_points = points.iter().filter(|x| is_a(x));
    let b_points: Vec<&Point> = points.iter().filter(|x| is_b(x)).collect();
    let mut max_diff: Vec<i32> = a_points
        .map(|a| {
            *&b_points.iter().fold(0, |acc, b| {
                let diff = distance(a, b);
                if diff > acc { diff } else { acc }
            })
        })
        .collect();
    max_diff.sort();
    match max_diff.pop() {
        Some(x) => x,
        None => 0,
    }
}

fn main() {
    let proj = proj_dir();
    let file = proj.join("elvish_cheat_codes.txt");
    let mut input = String::new();
    let _io = std::fs::File::open(file).unwrap().read_to_string(
        &mut input,
    );
    let points = get_points(&input);
    let f = furthest(&points);
    println!("Furthest: {}", f);
    let p = pair(&points);
    println!("Pair: {}", p);
}
