
use std::collections::HashMap;
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

fn hash_get(hm: &HashMap<Point, i64>, p: &Point) -> i64 {
    match hm.get(p) {
        Some(n) => *n,
        None => 0,
    }
}

fn adj(p: &Point, hm: &HashMap<Point, i64>) -> i64 {
    hash_get(hm, p)
        + hash_get(hm, &right(p))
        + hash_get(hm, &left(p))
        + hash_get(hm, &up(p))
        + hash_get(hm, &down(p))
        + hash_get(hm, &right(&up(p)))
        + hash_get(hm, &left(&up(p)))
        + hash_get(hm, &right(&down(p)))
        + hash_get(hm, &left(&down(p)))
}

fn more_than(n: i64) -> i64 {
    let mut ret = Point { x:0, y:0 };
    let mut all: HashSet<Point> = HashSet::new();
    let mut hash: HashMap<Point, i64> = HashMap::new();
    all.insert(ret.clone());
    hash.insert(ret.clone(), 1);
    let mut cur = hash_get(&hash, &ret);
    while cur < n {
        ret = next(&ret, &all);
        all.insert(ret.clone());
        let val = adj(&ret, &hash);
        hash.insert(ret.clone(), val);
        cur = hash_get(&hash, &ret);
    };
    cur
}

fn main() {
    let one = nth(325489);
    println!("Distance: {}", manhattan(&one, &Point {x:0, y:0 }));
    let two = more_than(325489);
    println!("More than: {}", two);
}

#[test]
fn test() {
    let one = nth(325489);
    let d = manhattan(&one, &Point { x:0, y:0 });
    assert_eq!(552, d);
    let two = more_than(325489);
    assert_eq!(330785, two);
}
