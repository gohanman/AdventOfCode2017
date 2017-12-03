
use std::collections::HashSet;

#[derive(PartialEq, Eq, Debug, Hash, Clone)]
struct Point {
    x: i64,
    y: i64,
}

fn left(p: &Point) -> Point {
    Point { x: p.x - 1, y: p.y }
}

fn right(p: &Point) -> Point {
    Point { x: p.x + 1, y: p.y }
}

fn up(p: &Point) -> Point {
    Point { x: p.x, y: p.y + 1 }
}

fn down(p: &Point) -> Point {
    Point { x: p.x, y: p.y - 1 }
}

fn next(p: &Point, hs: &HashSet<Point>) -> Point{
    let l = hs.contains(&left(p));
    let r = hs.contains(&right(p));
    let u = hs.contains(&up(p));
    let d = hs.contains(&down(p));
    if !l && !r && !u && !d {
        right(p)
    } else if l && !u {
        up(p)
    } else if d && !l {
        left(p)
    } else if r && !d {
        down(p)
    } else if u && !r {
        right(p)
    } else {
        Point { x:p.x, y:p.y }
    }
}

fn nth(n: i64) -> Point {
    let mut ret = Point { x:0, y:0 };
    let mut all: HashSet<Point> = HashSet::new();
    all.insert(ret.clone());
    let mut cur = 1;
    while cur < n {
        ret = next(&ret, &all);
        all.insert(ret.clone());
        cur += 1;
    };
    ret
}

fn manhattan(a: &Point, b: &Point) -> i64 {
    let x = a.x - b.x;
    let y = a.y - b.y;
    x.abs() + y.abs()
}

fn main() {
    let one = nth(325489);
    println!("Distance: {}", manhattan(&one, &Point {x:0, y:0 }));
}

#[test]
fn test() {
    let origin = Point { x:0, y:0 };
    assert_eq!(Point { x:-1, y:0 }, left(&origin));
    assert_eq!(Point { x:1, y:0 }, right(&origin));
    assert_eq!(Point { x:0, y:1 }, up(&origin));
    assert_eq!(Point { x:0, y:-1 }, down(&origin));
    assert_eq!(2, manhattan(&origin, &Point{ x: -1, y: 1 }));
    assert_eq!(Point { x:0, y:2 }, nth(15));
    assert_eq!(Point { x:1, y:-2 }, nth(24));
}
