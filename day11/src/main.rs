
extern crate proj_self;

#[derive(Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

enum Dir {
    North,
    NorthEast,
    SouthEast,
    South,
    SouthWest,
    NorthWest,
    Invalid,
}

fn str_to_dir(s: &str) -> Dir {
    match s {
        "n" => Dir::North,
        "ne" => Dir::NorthEast,
        "se" => Dir::SouthEast,
        "s" => Dir::South,
        "sw" => Dir::SouthWest,
        "nw" => Dir::NorthWest,
        _ => Dir::Invalid,
    }
}

fn next(p: &Point, d: &Dir) -> Point {
    match *d {
        Dir::North => Point { x: p.x, y: p.y + 2 },
        Dir::NorthEast => Point { x: p.x + 1, y: p.y + 1 },
        Dir::SouthEast => Point { x: p.x + 1, y: p.y - 1 },
        Dir::South => Point { x: p.x, y: p.y - 2 },
        Dir::SouthWest => Point { x: p.x - 1, y: p.y - 1 },
        Dir::NorthWest => Point { x: p.x - 1, y: p.y + 1 },
        Dir::Invalid => Point { x: p.x, y: p.y },
    }
}

fn diagonal(p1: &Point, p2: &Point) -> bool {
    (p1.x - p1.y) == (p2.x - p2.y)
}

fn walk(p: &Point, dirs: &Vec<Dir>) -> (Point, i32) {
    let mut ret = Point { x: p.x, y: p.y };
    let mut max = 0;
    for d in dirs {
        ret = next(&ret, &d);
        let dist = shortest(p, &ret);
        if dist > max {
            max = dist;
        }
    }

    (ret, max)
}

fn shortest(p1: &Point, p2: &Point) -> i32 {
    if *p1 == *p2 {
        return 0;
    }
    if p1.x == p2.x {
        return (p1.y - p2.y).abs() / 2;
    }

    if diagonal(p1, p2) {
        return (p1.x - p2.x).abs();
    }

    let mut vert = Point { x: p1.x, y: p1.y };
    let mut diag = Point { x: p1.x, y: p1.y };
    let mut steps = 0;
    loop {
        if vert.y < p2.y {
            vert = next(&vert, &Dir::North);
        } else {
            vert = next(&vert, &Dir::South);
        }
        if diag.x < p2.x {
            if diag.y < p2.y {
                diag = next(&diag, &Dir::NorthEast);
            } else {
                diag = next(&diag, &Dir::SouthEast);
            }
        } else {
            if diag.y < p2.y {
                diag = next(&diag, &Dir::NorthWest);
            } else {
                diag = next(&diag, &Dir::SouthWest);
            }
        }
        steps += 1;
        if diag == *p2 || vert == *p2 {
            return steps;
        }
        if diag.x == p2.x {
            return steps + shortest(&diag, p2);
        }
        if diagonal(&vert, p2) {
            return steps + shortest(&vert, p2);
        }
    }
}

fn main() {
    let proj = proj_self::proj_dir(3);
    let file = proj.join("input.txt");
    let input = proj_self::file_to_str(&file);
    let steps = input.trim().split(",").map(|x| str_to_dir(x)).collect();
    let origin = Point { x:0, y:0 };
    let (dest, max) = walk(&origin, &steps);
    println!("Finished at {}, {}", dest.x, dest.y);
    let dist = shortest(&origin, &dest);
    println!("Shortest path: {} steps", dist);
    println!("Max distance: {} steps", max);
}

#[test]
fn test() {
    let origin = Point { x:0, y:0 };
    let a = "ne,ne,ne".split(",").map(|x| str_to_dir(x)).collect();
    let (b, _) = walk(&origin, &a);
    assert_eq!(3, shortest(&origin, &b));
    let c = "ne,ne,sw,sw".split(",").map(|x| str_to_dir(x)).collect();
    let (d, _) = walk(&origin, &c);
    assert_eq!(0, shortest(&origin, &d));
    let e = "ne,ne,s,s".split(",").map(|x| str_to_dir(x)).collect();
    let (f, _) = walk(&origin, &e);
    assert_eq!(2, shortest(&origin, &f));
    let g = "se,sw,se,sw,sw".split(",").map(|x| str_to_dir(x)).collect();
    let (h, _) = walk(&origin, &g);
    assert_eq!(3, shortest(&origin, &h));
}
