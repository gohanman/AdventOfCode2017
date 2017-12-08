
extern crate proj_self;

enum Operator {
    Eq,
    NotEq,
    Gt,
    Gte,
    Lt,
    Lte,
    Invalid,
}

enum IncDec {
    Inc,
    Dec,
}

#[derive(Debug)]
struct CPU {
    registers: std::collections::HashMap<String, i64>,
    ptr: usize,
}

struct Conditional {
    register: String,
    op: Operator,
    operand: i64,
}

struct Instruction {
    register: String,
    inc: IncDec,
    amount: i64,
    condition: Conditional,
}

fn line_to_instruction(line: &str) -> Instruction {
    let parts: Vec<_> = line.split(' ').collect();
    let i_reg = parts[0].to_string();
    let inc = if parts[1] == "inc" { IncDec::Inc } else { IncDec::Dec };
    let i_amount = parts[2].parse().unwrap();
    let c_reg = parts[4].to_string();
    let c_op = match parts[5] {
        "==" => Operator::Eq,
        "!=" => Operator::NotEq,
        ">" => Operator::Gt,
        ">=" => Operator::Gte,
        "<" => Operator::Lt,
        "<=" => Operator::Lte,
        _ => Operator::Invalid,
    };
    let c_amount = parts[6].parse().unwrap();
    let c = Conditional { register: c_reg, op: c_op, operand: c_amount };
    Instruction { register: i_reg, inc: inc, amount: i_amount, condition: c }
}

fn get_register(cpu: &CPU, reg: &String) -> i64 {
    match cpu.registers.get(reg) {
        Some(x) => *x,
        None => 0,
    }
}

fn inc(inc: &IncDec, a: i64, b: i64) -> i64 {
    match *inc {
        IncDec::Inc => a + b,
        IncDec::Dec => a - b,
    }
}

fn cond(cpu: &CPU, c: &Conditional) -> bool {
    let val = get_register(cpu, &c.register);
    match c.op {
        Operator::Eq => val == c.operand,
        Operator::NotEq => val != c.operand,
        Operator::Gt => val > c.operand,
        Operator::Gte => val >= c.operand,
        Operator::Lt => val < c.operand,
        Operator::Lte => val <= c.operand,
        Operator::Invalid => false,
    }
}

fn process(cpu: &mut CPU, i: &Instruction) {
    if cond(cpu, &i.condition) {
        let newval = inc(&i.inc, get_register(cpu, &i.register), i.amount);
        cpu.registers.insert(i.register.clone(), newval);
    }
    cpu.ptr += 1;
}

fn run(instructions: &Vec<Instruction>) -> (CPU, i64) {
    let mut cpu = CPU { registers: std::collections::HashMap::new(), ptr: 0 };
    let mut max = 0;
    while cpu.ptr < instructions.len() {
        let cur = cpu.ptr;
        process(&mut cpu, &instructions[cur]);
        let cur_max = largest(&cpu);
        if cur_max > max {
            max = cur_max;
        }
    }
    (cpu, max)
}

fn largest(cpu: &CPU) -> i64 {
    let vals: Vec<&i64> = cpu.registers.values().collect();
    if vals.len() == 0 {
        return 0;
    }
    let mut max = *vals[0];
    for i in vals {
        if *i > max {
            max = *i;
        }
    }

    max
}

fn main() {
    let proj = proj_self::proj_dir(3);
    let file = proj.join("input.txt");
    let input = proj_self::file_to_str(&file);
    let lines = proj_self::str_to_lines(&input);
    let inst = lines.iter().map(|x| line_to_instruction(x)).collect();
    let (result, profiled) = run(&inst);
    let max = largest(&result);
    println!("Current max register value: {}", max);
    println!("Overall max register value: {}", profiled);
}

#[test]
fn test() {
    let lines = vec![
        "b inc 5 if a > 1",
        "a inc 1 if b < 5",
        "c dec -10 if a >= 1",
        "c inc -20 if c == 10",
    ];
    let inst: Vec<Instruction> = lines.iter().map(|x| line_to_instruction(x)).collect();
    let (result, profiled) = run(&inst);
    let max = largest(&result);
    assert_eq!(max, 1);
    assert_eq!(profiled, 10);
}
