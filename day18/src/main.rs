
#[derive(Clone, Debug)]
enum Operand {
    Value(i64),
    Register(char),
}

#[derive(Clone, Debug)]
enum ISA {
    Play(char),
    Recover(char),
    Send(char),
    Recv(char),
    Set(char, Operand),
    Add(char, Operand),
    Mul(char, Operand),
    Mod(char, Operand),
    JumpGZ(Operand, Operand),
    Invalid,
}

struct State {
    registers: std::collections::HashMap<char, i64>,
    program: Vec<ISA>,
    pc: i64,
    last_freq: i64,
    last_recover: i64,
    recover_count: i64,
    waiting: bool,
    terminated: bool,
    mailbox: std::collections::VecDeque<i64>,
    send_count: u64,
    recv_count: u64,
}
impl State {
    fn new(instructions: &Vec<ISA>) -> State {
        State { 
            registers: std::collections::HashMap::new(),
            program: instructions.clone().to_vec(), 
            pc: 0,
            last_freq: 0,
            last_recover: 0,
            recover_count: 0,
            waiting: false,
            mailbox: std::collections::VecDeque::new(),
            terminated: false,
            send_count: 0,
            recv_count: 0,
        }
    }
}

fn get_reg(x: char, state: &State) -> i64 {
    *state.registers.get(&x).unwrap_or(&0)
}

fn opr_to_val(opr: &Operand, state: &State) -> i64 {
    match *opr {
        Operand::Value(x) => x,
        Operand::Register(x) => get_reg(x, state),
    }
}

fn str_to_opr(s: &str) -> Operand {
    match s.trim().parse() {
        Ok(x) => Operand::Value(x),
        Err(_e) => {
            let mut as_str = s.trim().to_string();
            Operand::Register(as_str.remove(0))
        }
    }
}

fn str_to_isa(s: &str, sounds: bool) -> ISA {
    let parts: Vec<_> = s.split(" ").collect();
    let mut tmp = parts[1].trim().to_string();
    let reg = tmp.remove(0);
    match parts[0] {
        "snd" => if sounds { ISA::Play(reg) } else { ISA::Send(reg) },
        "rcv" => if sounds { ISA::Recover(reg) } else { ISA::Recv(reg) },
        "set" => ISA::Set(reg, str_to_opr(&parts[2])),
        "add" => ISA::Add(reg, str_to_opr(&parts[2])),
        "mul" => ISA::Mul(reg, str_to_opr(&parts[2])),
        "mod" => ISA::Mod(reg, str_to_opr(&parts[2])),
        "jgz" => ISA::JumpGZ(str_to_opr(&parts[1]), str_to_opr(&parts[2])),
        _ => ISA::Invalid,
    }
}

fn parallel(instructions: &Vec<ISA>) {
    let mut state_a = State::new(instructions);
    state_a.registers.insert('p', 0);
    let mut state_b = State::new(instructions);
    state_b.registers.insert('p', 1);
    loop {
        state_a.waiting = false;
        state_b.waiting = false;

        while !state_a.waiting && !state_a.terminated {
            let msg_for_b = step(&mut state_a);
            match msg_for_b {
                Some(msg) => state_b.mailbox.push_back(msg),
                None => (),
            };
        };
        let b_count = state_b.send_count;
        while !state_b.waiting && !state_b.terminated {
            let msg_for_a = step(&mut state_b);
            match msg_for_a {
                Some(msg) => state_a.mailbox.push_back(msg),
                None => (),
            };
        };

        if b_count < state_b.send_count {
            continue;
        }

        if (state_a.waiting || state_a.terminated) && (state_b.waiting || state_b.terminated) {
            break;
        }
    };

    println!("Message stats:");
    println!("A sent {}, received {}", state_a.send_count, state_a.recv_count);
    println!("B sent {}, received {}", state_b.send_count, state_b.recv_count);
}

fn step(state: &mut State) -> Option<i64> {
    let mut ret = None;
    if state.terminated || state.pc < 0 || (state.pc as usize) > state.program.len() - 1 {
        println!("Terminated!");
        state.terminated = true;
        return ret;
    }

    match state.program[state.pc as usize] {
        ISA::Play(x) => {
            state.last_freq = get_reg(x, state);
        },
        ISA::Recover(x) => {
            let cur = get_reg(x, state);
            if cur != 0 {
                state.last_recover = state.last_freq;
                state.recover_count += 1;
            };
        },
        ISA::Set(x, ref y) => {
            let new_val = opr_to_val(&y, state);
            state.registers.insert(x, new_val);
        },
        ISA::Add(x, ref y) => {
            let added = get_reg(x, state) + opr_to_val(&y, state);
            state.registers.insert(x, added);
        },
        ISA::Mul(x, ref y) => {
            let mult = get_reg(x, state) * opr_to_val(&y, state);
            state.registers.insert(x, mult);
        },
        ISA::Mod(x, ref y) => {
            let div = get_reg(x, state) % opr_to_val(&y, state);
            state.registers.insert(x, div);
        },
        ISA::JumpGZ(ref x, ref y) => {
            if opr_to_val(x, state) > 0 {
                state.pc = state.pc + opr_to_val(&y, state) - 1;
            };
        },
        ISA::Send(x) => {
            ret = Some(get_reg(x, state));
            state.send_count += 1;
        },
        ISA::Recv(x) => {
            match state.mailbox.pop_front() {
                Some(msg) => {
                    state.registers.insert(x, msg);
                    state.recv_count += 1;
                },
                None => {
                    state.waiting = true;
                    state.pc -= 1;
                },
            };
        },
        _ => (),
    };
    state.pc += 1;
    ret
}

fn main() {
    let lines = vec![
        "set i 31",
        "set a 1",
        "mul p 17",
        "jgz p p",
        "mul a 2",
        "add i -1",
        "jgz i -2",
        "add a -1",
        "set i 127",
        "set p 622",
        "mul p 8505",
        "mod p a",
        "mul p 129749",
        "add p 12345",
        "mod p a",
        "set b p",
        "mod b 10000",
        "snd b",
        "add i -1",
        "jgz i -9",
        "jgz a 3",
        "rcv b",
        "jgz b -1",
        "set f 0",
        "set i 126",
        "rcv a",
        "rcv b",
        "set p a",
        "mul p -1",
        "add p b",
        "jgz p 4",
        "snd a",
        "set a b",
        "jgz 1 3",
        "snd b",
        "set f 1",
        "add i -1",
        "jgz i -11",
        "snd a",
        "jgz f -16",
        "jgz a -19",
    ];
    let instr: Vec<ISA> = lines.iter().map(|x| str_to_isa(x, true)).collect();
    let mut state = State::new(&instr);
    while state.recover_count == 0 {
        step(&mut state);
    }
    println!("First recovery: {}", state.last_recover);

    let msgs: Vec<ISA> = lines.iter().map(|x| str_to_isa(x, false)).collect();
    parallel(&msgs);
}

#[test]
fn test() {
    let lines = vec![
        "set a 1",
        "add a 2",
        "mul a a",
        "mod a 5",
        "snd a",
        "set a 0",
        "rcv a",
        "jgz a -1",
        "set a 1",
        "jgz a -2",
    ];
    let instr: Vec<ISA> = lines.iter().map(|x| str_to_isa(x, true)).collect();
    let mut state = State::new(&instr);
    while state.recover_count == 0 {
        step(&mut state);
    }
    assert_eq!(4, state.last_recover);

    let part2 = vec![
        "snd 1",
        "snd 2",
        "snd p",
        "rcv a",
        "rcv b",
        "rcv c",
        "rcv d",
    ];
    let messaging : Vec<ISA>  = part2.iter().map(|x| str_to_isa(x, false)).collect();
    parallel(&messaging);
}
