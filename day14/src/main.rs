
extern crate knot_hash;

#[derive(Eq, PartialEq)]
enum Status {
    Full,
    Empty,
    Marked,
}

fn new_region(s: &Status) -> bool {
    match *s {
        Status::Full => true,
        _ => false,
    }
}

fn adjacent(index: usize) -> Vec<usize> {
    let mut adj = vec![];
    let x = index % 128;
    let y = index / 128;
    if x > 0 {
        adj.push(index - 1);
    }
    if x < 127 {
        adj.push(index + 1);
    }
    if y > 0 {
        adj.push(index - 128);
    }
    if y < 127 {
        adj.push(index + 128);
    }

    adj
}

fn is_region(grid: &mut Vec<Status>, index: usize) -> bool {
    if !new_region(&grid[index]) {
        return false;
    }

    grid[index] = Status::Marked;
    for i in adjacent(index) {
        is_region(grid, i);
    }

    true
}

fn get_hash(key: &str, row: usize) -> String {
    let real_key = format!("{}-{}", key, row);
    knot_hash::hash_string(&real_key)
}

fn hash_to_status(hash: &str) -> Vec<Status> {
    let mut status = vec![];
    for c in hash.chars() {
        match c {
            '8' | '9' | 'a' | 'b' | 'c' | 'd' | 'e' | 'f' => status.push(Status::Full),
            _ => status.push(Status::Empty),
        }
        match c {
            '4' | '5' | '6' | '7' | 'c' | 'd' | 'e' | 'f' => status.push(Status::Full),
            _ => status.push(Status::Empty),
        }
        match c {
            '2' | '3' | '6' | '7' | 'a' | 'b' | 'e' | 'f' => status.push(Status::Full),
            _ => status.push(Status::Empty),
        }
        match c {
            '1' | '3' | '5' | '7' | '9' | 'b' | 'd' | 'f' => status.push(Status::Full),
            _ => status.push(Status::Empty),
        }
    }

    status
}

fn long_hash(key: &str) -> String {
    let mut full_hash = String::new();
    for i in 0..128 {
        let hash = get_hash(key, i);
        full_hash.push_str(&hash);
    }

    full_hash
}

fn region_count(hash: &str) -> u32 {
    let mut count = 0;
    let mut grid = hash_to_status(hash);
    for i in 0..(128*128) {
        if is_region(&mut grid, i) {
            count += 1;
        }
    }

    count
}

fn bit_count(hash: &str) -> u32 {
    let mut bits = 0;
    for c in hash.chars() {
        bits += match c {
            '1' | '2' | '4' | '8' => 1,
            '3' | '5' | '6' | '9' | 'a' | 'c' => 2,
            '7' | 'b' | 'd' | 'e' => 3,
            'f' => 4,
            _ => 0,
        }
    }

    bits
}

fn main() {
    let mut bits = 0;
    for i in 0..128 {
        bits += bit_count(&get_hash("ffayrhll", i));
    }
    println!("Used: {}", bits);

    let hash = long_hash("ffayrhll");
    let regions = region_count(&hash);
    println!("Regions: {}", regions);
}

#[test]
fn test() {
    let mut test_bits = 0;
    for i in 0..128 {
        test_bits += bit_count(&get_hash("flqrgnkx", i));
    }
    assert_eq!(8108, test_bits);

    let hash = long_hash("flqrgnkx");
    let regions = region_count(&hash);
    assert_eq!(1242, regions);
}

