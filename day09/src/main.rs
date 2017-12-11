
extern crate proj_self;

#[derive(Debug, Clone, Copy)]
enum Sequence {
    Group,
    Garbage,
    None,
}

#[derive(Debug, Clone, Copy)]
struct State {
    nesting_level: u64,
    cur_seq: Sequence,
    ignore_flag: bool,
    score: u64,
    garbage: u64,
}

fn next(s: &State, c: &char) -> State{
    if s.ignore_flag {
        return State { ignore_flag:false, ..s.clone() };
    }

    match *c {
        '!' => State { ignore_flag:true, ..s.clone() },
        '<' => start_garbage(s),
        '>' => end_garbage(s),
        '{' => start_group(s),
        '}' => end_group(s),
        _   => match s.cur_seq {
            Sequence::Garbage => State { garbage: s.garbage + 1, ..s.clone() },
            _ => s.clone(),
        }
    }
}

fn start_garbage(s: &State) -> State {
    match s.cur_seq {
        Sequence::Garbage => State { garbage: s.garbage + 1, ..s.clone() },
        _ => State { cur_seq: Sequence::Garbage, ..s.clone() },
    }
}


fn end_garbage(s: &State) -> State {
    match s.cur_seq {
        Sequence::Garbage => State { cur_seq: Sequence::Group, ..s.clone() },
        _ => s.clone(),
    }
}

fn start_group(s: &State) -> State {
    match s.cur_seq {
        Sequence::Garbage => State { garbage: s.garbage + 1, ..s.clone() },
        Sequence::Group => State { nesting_level: s.nesting_level + 1, cur_seq: Sequence::Group, ..s.clone() },
        Sequence::None => State { cur_seq: Sequence::Group, ..s.clone() }
    }
}

fn end_group(s: &State) -> State {
    match s.cur_seq {
        Sequence::Group => State {
                nesting_level: s.nesting_level - 1,
                score: s.score + s.nesting_level,
                ..s.clone()
            },
        Sequence::Garbage => State { garbage: s.garbage + 1, ..s.clone() },
        Sequence::None => s.clone(),
    }
}

fn score_string(s: &str) -> (u64, u64) {
    let mut state = State { nesting_level: 1, cur_seq: Sequence::None, ignore_flag: false, score: 0, garbage: 0 };
    for c in s.chars() {
        //println!("{:?}", state);
        state = next(&state, &c);
    }
    //println!("{:?}", state);

    (state.score, state.garbage)
}

fn main() {
    let proj = proj_self::proj_dir(3);
    let file = proj.join("input.txt");
    let input = proj_self::file_to_str(&file);
    let (score, garbage) = score_string(&input.trim());
    println!("Score: {}", score);
    println!("Garbage: {}", garbage);
}

#[test]
fn test() {
    assert_eq!((1, 0), score_string("{}"));
    assert_eq!((6, 0), score_string("{{{}}}"));
    assert_eq!((5, 0), score_string("{{},{}}"));
    assert_eq!((16, 0), score_string("{{{},{},{{}}}}"));
    assert_eq!((1, 4), score_string("{<a>,<a>,<a>,<a>}"));
    assert_eq!((9, 8), score_string("{{<ab>},{<ab>},{<ab>},{<ab>}}"));
    assert_eq!((9, 0), score_string("{{<!!>},{<!!>},{<!!>},{<!!>}}"));
    assert_eq!((3, 17), score_string("{{<a!>},{<a!>},{<a!>},{<ab>}}"));
}
