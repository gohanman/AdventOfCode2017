
#[derive(Debug)]
struct Generator {
    prev: u64,
    factor: u64,
}

impl Generator {
    pub fn new(f: u64, seed: u64) -> Generator {
        Generator { prev: seed, factor: f }
    }
}

impl Iterator for Generator {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        let ret = (self.prev * self.factor) % 2147483647;
        self.prev = ret;

        Some(ret)
    }
}

fn judge_match(a: u64, b: u64) -> bool {
    (a & 0xffff) == (b & 0xffff)
}

fn count_matches(rounds: u64, seed_a: u64, seed_b: u64) -> u64 {
    let mut matches = 0;
    let mut gen_a = Generator::new(16807, seed_a);
    let mut gen_b = Generator::new(48271, seed_b);
    for _i in 0..rounds {
        let a = gen_a.next().unwrap();
        let b = gen_b.next().unwrap();
        if judge_match(a, b) {
            matches += 1;
        }
    }

    matches
}

fn picky_matches(rounds: u64, seed_a: u64, seed_b: u64) -> u64 {
    let mut matches = 0;
    let mut gen_a = Generator::new(16807, seed_a);
    let mut gen_b = Generator::new(48271, seed_b);
    for _i in 0..rounds {
        let mut a = gen_a.next().unwrap();
        while a % 4 != 0 {
            a = gen_a.next().unwrap();
        }
        let mut b = gen_b.next().unwrap();
        while b % 8 != 0 {
            b = gen_b.next().unwrap();
        }
        if judge_match(a, b) {
            matches += 1;
        }
    }

    matches
}

fn main() {
    let pairs = count_matches(40000000, 722, 354);
    println!("Matches: {}", pairs);
    let pairs2 = picky_matches(5000000, 722, 354);
    println!("Matches: {}", pairs2);
}

#[test]
fn test() {
    assert_eq!(1, count_matches(5, 65, 8921));
    assert_eq!(588, count_matches(40000000, 65, 8921));
    assert_eq!(0, picky_matches(5, 65, 8921));
    assert_eq!(309, picky_matches(5000000, 65, 8921));
}
