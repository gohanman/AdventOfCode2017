
#[derive(PartialEq, Eq, Debug)]
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

fn manhattan(a: &Point, b: &Point) -> i64 {
    let x = a.x - b.x;
    let y = a.y - b.y;
    x.abs() + y.abs()
}

fn main() {
    println!("Hello, world!");
}

#[test]
fn test() {
    let origin = Point { x:0, y:0 };
    assert_eq!(Point { x:-1, y:0 }, left(&origin));
    assert_eq!(Point { x:1, y:0 }, right(&origin));
    assert_eq!(Point { x:0, y:1 }, up(&origin));
    assert_eq!(Point { x:0, y:-1 }, down(&origin));
    assert_eq!(2, manhattan(&origin, &Point{ x: -1, y: 1 }));
}
