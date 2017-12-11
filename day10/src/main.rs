
use std::fmt::Write;

struct State {
    list: Vec<u8>,
    skip_size: u32,
    pos: usize,
}

impl State {
    pub fn new(size: usize) -> State {
        let mut arr = vec![0 as u8; size];
        for i in 0..size {
            arr[i] = i as u8;
        }
        State { list: arr, skip_size: 0, pos: 0 }
    }

    pub fn twist(&mut self, length: u8) {
        let mut tmp = vec![0; length as usize];
        for i in 0..(length as usize) {
            let j = (self.pos + i) % self.list.len();
            tmp[i] = self.list[j];
        }
        tmp.reverse();
        for i in 0..(length as usize) {
            let j = (self.pos + i) % self.list.len();
            self.list[j] = tmp[i];
        }
        self.pos = self.pos + (length as usize) + (self.skip_size as usize);
        self.skip_size += 1;
    }

    pub fn value(&self) -> u32 {
        if self.list.len() < 2 { 0 } else { (self.list[0] as u32) * (self.list[1] as u32) }
    }

    pub fn dense(&self) -> Vec<u8> {
        let mut v = vec![];
        let groups = self.list.len() / 16;
        for i in 0..groups {
            let mut val = self.list[i*16];
            let min = (i*16) + 1;
            let max = (i+1) * 16;
            for j in min..max {
                val = val ^ self.list[j];
            }
            v.push(val);
        }

        v
    }
}

fn str_to_ascii(s: &str) -> Vec<u8> {
    let mut v = vec![];
    for b in s.trim().to_string().into_bytes() {
       v.push(b); 
    }
    for b in vec![17, 31, 73, 47, 23] {
        v.push(b);
    }

    v
}

fn hex(bytes: &Vec<u8>) -> String {
    let mut s = String::new();
    for b in bytes {
        write!(s, "{:02x}", b).expect("wtf");
    }
    s
}

fn hash_string(s: &str) -> String{
    let lengths = str_to_ascii(s);
    let mut state = State::new(256);
    for _i in 0..64 {
        for l in &lengths {
            state.twist(*l);
        }
    }
    hex(&state.dense())
}

fn main() {
    let lengths = vec![157,222,1,2,177,254,0,228,159,140,249,187,255,51,76,30];
    let mut s = State::new(256);
    for l in lengths {
        s.twist(l);
    }
    println!("Value: {}", s.value());
    let f = hash_string("157,222,1,2,177,254,0,228,159,140,249,187,255,51,76,30");
    println!("Final: {}", f);
}

#[test]
fn test() {
    let mut s = State::new(5);
    let lengths = vec![3, 4, 1, 5];
    for l in lengths {
        s.twist(l);
    }
    assert_eq!(12, s.value());

    assert_eq!("a2582a3a0e66e6e86e3812dcb672a272".to_string(), hash_string(""));
    assert_eq!("33efeb34ea91902bb2f59c9920caa6cd".to_string(), hash_string("AoC 2017"));
    assert_eq!("3efbe78a8d82f29979031a4aa0b16a9d".to_string(), hash_string("1,2,3"));
    assert_eq!("63960835bcdc130f0b66d7ff4f6a5a8e".to_string(), hash_string("1,2,4"));
}
