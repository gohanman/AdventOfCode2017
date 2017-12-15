
extern crate knot_hash;

use knot_hash::State;
use knot_hash::hash_string;

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
