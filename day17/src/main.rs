
#[derive(Debug)]
struct SpinLock {
    pos: usize,
    step: usize,
    buffer: Vec<usize>,
}

fn next(sp: &SpinLock, val: usize) -> SpinLock {
    let insert_pos = (sp.pos + sp.step) % sp.buffer.len();
    let mut new_buf = vec![];
    for i in 0..sp.buffer.len() {
        new_buf.push(sp.buffer[i]);
        if i == insert_pos {
            new_buf.push(val);
        }
    }

    SpinLock { pos: insert_pos+1, step: sp.step, buffer: new_buf }
}

fn main() {
    let mut sp = SpinLock { pos: 0, step: 303, buffer: vec![0; 1] };
    let mut second = 0;
    for i in 1..2018 {
        sp = next(&sp, i);
        if second != sp.buffer[1] {
            println!("1, 2: {}, {}", sp.buffer[0], sp.buffer[1]);
            second = sp.buffer[1];
        }
    }
    println!("Next: {}", sp.buffer[sp.pos + 1]);

    let mut len = 1;
    let mut pos = 0;
    let mut ans = 0;
    for i in 0..50000000 {
        let next_val = i + 1; 
        let next_pos = ((pos + 303) % len) + 1;
        if next_pos == 1 {
            ans = next_val;
        }
        pos = next_pos;
        len += 1;
    }
    println!("Next to zero: {}", ans);
}

#[test]
fn test() {
    let mut sp = SpinLock { pos: 0, step: 3, buffer: vec![0; 1] };
    for i in 1..2018 {
        sp = next(&sp, i);
    }
    assert_eq!(2017, sp.buffer[sp.pos]);
    assert_eq!(638, sp.buffer[sp.pos + 1]);
}
