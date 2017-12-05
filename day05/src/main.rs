
extern crate proj_self;

#[derive(Debug)]
struct World {
    jumps: Vec<i64>,
    pos: i64,
    solved: bool,
}

fn inc(i: i64) -> i64 {
    i + 1
}

fn incdec(i: i64) -> i64 {
    if i >= 3 { i - 1 } else { i + 1 }
}

fn jump(w: &World, f: &Fn(i64) -> i64) -> World {
    if w.pos < 0 || w.pos >= (w.jumps.len() as i64) {
        return World { solved: true, pos: w.pos, jumps: w.jumps.clone() };
    }
    let new_pos = w.pos + w.jumps[(w.pos as usize)];
    let mut new_vec = w.jumps.clone();
    new_vec[(w.pos as usize)] = f(w.jumps[w.pos as usize]);

    World { solved: w.solved, pos: new_pos, jumps: new_vec }
}

fn solve(w: &World, f: &Fn(i64) -> i64) -> i64 {
    let mut counter = 0;
    let mut state = World { solved: w.solved, pos: w.pos, jumps: w.jumps.clone() };
    loop {
        if state.solved {
            break;
        }
        state = jump(&state, f);
        counter += 1;
    }

    counter - 1
}

fn main() {
    let proj = proj_self::proj_dir(3);
    let file = proj.join("input.txt");
    let input = proj_self::file_to_str(&file);
    let vals = proj_self::str_to_lines(&input);
    let ivals: Vec<i64> = vals.iter().map(|x| x.parse().unwrap()).collect();
    let w = World { jumps: ivals, pos: 0, solved: false };
    let steps = solve(&w, &inc);
    println!("Steps: {}", steps);
    let steps2 = solve(&w, &incdec);
    println!("Steps: {}", steps2);
}

#[test]
fn test() {
    let j = vec![0, 3, 0, 1, -3];
    let w = World { jumps: j, pos: 0, solved: false };
    let steps = solve(&w, &inc);
    assert_eq!(5, steps);
    let steps2 = solve(&w, &incdec);
    assert_eq!(10, steps2);
}
