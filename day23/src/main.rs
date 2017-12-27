
extern crate proj_self;

#[derive(Debug, Eq, PartialEq, Clone)]
enum IValue {
    Register(char),
    Value(i64),
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum ISA {
    Set(char,IValue), 
    Sub(char,IValue),
    Mul(char,IValue),
    Jnz(IValue,IValue),
    Jpr(IValue,IValue),
    Invalid,
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct CoProc {
    registers: std::collections::HashMap<char,i64>,
    pc: usize,
    instructions: Vec<ISA>,
    halt: bool,
    mult: usize,
}
impl CoProc {
    fn new(inst: &Vec<ISA>) -> CoProc {
        CoProc { registers: std::collections::HashMap::new(), pc: 0, 
            instructions: inst.clone(), halt: false, mult: 0 }
    }

    fn get_val(&self, ival: &IValue) -> i64 {
        match *ival {
            IValue::Value(i) => i,
            IValue::Register(r) => {
                match self.registers.get(&r) {
                    Some(v) => *v,
                    None => 0,
                }
            }
        }
    }

    fn set_reg(&mut self, r: char, i: i64) {
        self.registers.insert(r, i);
    }

    fn step(&mut self) {
        if self.pc < 0 || self.pc >= self.instructions.len() {
            self.halt = true;
            return ();
        }
        match self.instructions[self.pc] {
            ISA::Set(c, ref ival) => {
                let val = self.get_val(&ival);
                self.registers.insert(c, val);
            },
            ISA::Sub(c, ref ival) => {
                let val = self.get_val(&ival);
                let reg = self.get_val(&IValue::Register(c));
                self.registers.insert(c, reg - val);
            },
            ISA::Mul(c, ref ival) => {
                let val = self.get_val(&ival);
                let reg = self.get_val(&IValue::Register(c));
                self.registers.insert(c, reg * val);
                self.mult += 1;
            },
            ISA::Jnz(ref aval, ref bval) => {
                let tst = self.get_val(&aval);
                let jmp = self.get_val(&bval);
                if tst != 0 {
                    let next = (self.pc as i64) + jmp - 1;
                    if next < 0 {
                        self.halt = true;
                        self.pc = self.instructions.len() + 1;
                    } else {
                        self.pc = next as usize;
                    }
                };
            },
            ISA::Jpr(ref aval, ref bval) => {
                let tst = self.get_val(&aval);
                let jmp = self.get_val(&bval);
                if is_prime(tst) {
                    let next = (self.pc as i64) + jmp - 1;
                    if next < 0 {
                        self.halt = true;
                        self.pc = self.instructions.len() + 1;
                    } else {
                        self.pc = next as usize;
                    }
                };
            },
            _ => (),
        };
        self.pc += 1;
    }
}

fn is_prime(i: i64) -> bool {
    if i > 2 && i % 2 == 0 {
        return false;
    }
    let max = (i / 2) + 1;
    for j in 3..max {
        if i % j == 0 {
            return false;
        }
    }

    true
}

fn str_to_ival(s: &str) -> IValue {
    match s.parse() {
        Ok(i) => IValue::Value(i),
        Err(_e) => {
            let c = s.chars().nth(0).unwrap();
            IValue::Register(c)
        }
    }
}

fn str_to_isa(s: &str) -> ISA {
    let parts: Vec<_> = s.split(' ').collect();
    match parts[0] {
        "set" => {
            let c = parts[1].chars().nth(0).unwrap();
            ISA::Set(c, str_to_ival(parts[2]))
        },
        "sub" => {
            let c = parts[1].chars().nth(0).unwrap();
            ISA::Sub(c, str_to_ival(parts[2]))
        },
        "mul" => {
            let c = parts[1].chars().nth(0).unwrap();
            ISA::Mul(c, str_to_ival(parts[2]))
        },
        "jnz" => ISA::Jnz(str_to_ival(parts[1]), str_to_ival(parts[2])),
        "jpr" => ISA::Jpr(str_to_ival(parts[1]), str_to_ival(parts[2])),
        _ => ISA::Invalid,
    }
}

fn main() {
    let proj = proj_self::proj_dir(3);
    let file = proj.join("input.txt");
    let input = proj_self::file_to_str(&file);
    let lines = proj_self::str_to_lines(&input);

    let isa: Vec<ISA> = lines.iter().map(|x| str_to_isa(x)).collect();
    let mut coproc = CoProc::new(&isa);
    loop {
        coproc.step();
        if coproc.halt {
            break;
        }
    };
    println!("MULs {}", coproc.mult);

    let file2 = proj.join("optimize.txt");
    let input2 = proj_self::file_to_str(&file2);
    let lines2 = proj_self::str_to_lines(&input2);

    let isa2: Vec<ISA> = lines2.iter().map(|x| str_to_isa(x)).collect();
    let mut coproc2 = CoProc::new(&isa2);
    coproc2.set_reg('a', 1);
    loop {
        //println!("PC: {}, H: {}", coproc2.pc, coproc2.get_val(&IValue::Register('h')));
        coproc2.step();
        if coproc2.halt {
            break;
        }
    };
    println!("H: {}", coproc2.get_val(&IValue::Register('h')));
}
