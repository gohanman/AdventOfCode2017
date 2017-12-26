
#[macro_use] extern crate lazy_static;
extern crate regex;
extern crate proj_self;
use regex::Regex;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Particle {
    position: Point,
    velocity: Point,
    acceleration: Point,
    collided: bool,
}
impl Particle {
    pub fn new(p: (i32,i32,i32), v: (i32,i32,i32), a: (i32,i32,i32)) -> Particle {
        let pos = Point { x: p.0, y: p.1, z: p.2 };
        let vel = Point { x: v.0, y: v.1, z: v.2 };
        let acc = Point { x: a.0, y: a.1, z: a.2 };

        Particle { position: pos, velocity: vel, acceleration: acc, collided: false }
    }

    pub fn from_str(s: &str) -> Particle {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^\s*p=<([0-9-]+),([0-9-]+),([0-9-]+)>,\s*v=<([0-9-]+),([0-9-]+),([0-9-]+)>,\s*a=<([0-9-]+),([0-9-]+),([0-9-]+)>\s*$").unwrap();
        }
        let caps = RE.captures(s).unwrap();

        let p1 = caps.get(1).unwrap().as_str().parse().unwrap();
        let p2 = caps.get(2).unwrap().as_str().parse().unwrap();
        let p3 = caps.get(3).unwrap().as_str().parse().unwrap();
        let pos = Point { x: p1, y: p2, z: p3 };
        let v1 = caps.get(4).unwrap().as_str().parse().unwrap();
        let v2 = caps.get(5).unwrap().as_str().parse().unwrap();
        let v3 = caps.get(6).unwrap().as_str().parse().unwrap();
        let vel = Point { x: v1, y: v2, z: v3 };
        let a1 = caps.get(7).unwrap().as_str().parse().unwrap();
        let a2 = caps.get(8).unwrap().as_str().parse().unwrap();
        let a3 = caps.get(9).unwrap().as_str().parse().unwrap();
        let acc = Point { x: a1, y: a2, z: a3 };

        Particle { position: pos, velocity: vel, acceleration: acc, collided: false }
    }
}

fn add(p1: &Point, p2: &Point) -> Point {
    Point { x: p1.x + p2.x, y: p1.y + p2.y, z: p1.z + p2.z }
}

fn tick(p: &Particle) -> Particle {
    if p.collided {
        return p.clone();
    }
    let v = add(&p.velocity, &p.acceleration);
    let pos = add(&p.position, &v);
    Particle { position: pos, velocity: v, acceleration: p.acceleration.clone(), collided: p.collided }
}

fn mark_collisions(pts: &mut Vec<Particle>) {
    for i in 0..pts.len() {
        if pts[i].collided {
            continue;
        }
        for j in (i+1)..pts.len() {
            if !pts[j].collided && pts[i].position == pts[j].position {
                pts[i].collided = true;
                pts[j].collided = true;
            }
        }
    }
}

fn still_flying(particles: &Vec<Particle>) -> usize {
    let mut ret = 0;
    for i in particles.iter() {
        if !i.collided {
            ret += 1;
        }
    }

    ret
}

fn dist(p1: &Point, p2: &Point) -> i32 {
    let xdiff = p1.x - p2.x;
    let ydiff = p1.y - p2.y;
    let zdiff = p1.z - p2.z;

    xdiff.abs() + ydiff.abs() + zdiff.abs()
}

fn closest(particles: &Vec<Particle>) -> (Vec<usize>, Vec<usize>) {
    let mut min = std::i32::MAX;
    let mut accel = std::i32::MAX;
    let mut closest = vec![];
    let mut slowest = vec![];
    let origin = Point { x: 0, y: 0, z: 0 };
    for (i, p) in particles.iter().enumerate() {
        let d = dist(&p.position, &origin);
        if d < min {
            closest = vec![];
            closest.push(i);
            min = d;
        } else if d == min {
            closest.push(i);
        }
        let a = dist(&p.acceleration, &origin);
        if a < accel {
            slowest = vec![];
            slowest.push(i);
            accel = a;
        } else if a == accel {
            slowest.push(i);
        }
    }

    (closest, slowest)
}

fn main() {
    let proj = proj_self::proj_dir(3);
    let file = proj.join("input.txt");
    let input = proj_self::file_to_str(&file);
    let lines = proj_self::str_to_lines(&input);
    let mut particles: Vec<Particle> = lines.iter().map(|x| Particle::from_str(x)).collect();
    let done;
    loop {
        particles = particles.iter().map(|x| tick(x)).collect();
        let (close, slow) = closest(&particles);
        if close.len() == 1 && slow.len() == 1 && close[0] == slow[0] {
            done = close[0];
            break;
        }
    }
    println!("Particle {} is closest", done);

    let mut parts2: Vec<Particle> = lines.iter().map(|x| Particle::from_str(x)).collect();
    let mut steps = 0;
    let all;
    loop {
        parts2 = parts2.iter().map(|x| tick(x)).collect();
        mark_collisions(&mut parts2);
        let remains = still_flying(&parts2);
        steps += 1;
        eprint!("{}\t{}\r", remains, steps);
        if steps > 50000 {
            all = remains;
            break;
        }
    }
    println!("Remaining: {}", all);
}

#[test]
fn test() {
    let mut particles = vec![
        Particle::new( (3,0,0), (2,0,0), (-1,0,0) ),
        Particle::new( (4,0,0), (0,0,0), (-2,0,0) ),
    ];
    let done;
    loop {
        particles = particles.iter().map(|x| tick(x)).collect();
        let (close, slow) = closest(&particles);
        if close.len() == 1 && slow.len() == 1 && close[0] == slow[0] {
            done = close[0];
            break;
        }
    }
    assert_eq!(done, 0);

    let p = Particle::from_str("p=<-3787,-3683,3352>, v=<41,-25,-124>, a=<5,9,1>");
    assert_eq!(p.acceleration.x, 5);
    assert_eq!(p.acceleration.z, 1);
    assert_eq!(p.position.y, -3683);

    let mut parts2 = vec![
        Particle::new( (-6,0,0), (3,0,0), (0,0,0) ),
        Particle::new( (-4,0,0), (2,0,0), (0,0,0) ),
        Particle::new( (-2,0,0), (1,0,0), (0,0,0) ),
        Particle::new( (3,0,0), (-1,0,0), (0,0,0) ),
    ];
    let mut steps = 0;
    let all;
    loop {
        parts2 = parts2.iter().map(|x| tick(x)).collect();
        mark_collisions(&mut parts2);
        let remains = still_flying(&parts2);
        steps += 1;
        if steps > 50 {
            all = remains;
            break;
        }
    }
    assert_eq!(all, 1);
}
