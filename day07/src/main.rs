
extern crate proj_self;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Program {
    name: String,
    weight: u32,
    tower: Vec<Program>,
    finished: bool,
}

fn new_program(name: &str) -> Program {
    Program { name: name.to_string(), weight: 0, tower: vec![], finished: false }
}

fn is_finished(programs: &Vec<Program>) -> bool {
    for p in programs {
        if !p.finished {
            return false;
        }
    }

    true
}

fn find_program(name: &str, programs: &Vec<Program>) -> Program {
    for p in programs {
        if p.name == name {
            return p.clone();
        }
    }

    new_program("")
}

fn set_all_weights(programs: &Vec<Program>) -> Vec<Program> {
    let mut ret = programs.clone();
    while !is_finished(&ret) {
        let so_far = ret.clone();
        for (i, p) in so_far.iter().enumerate() {
            if !p.finished {
                let mut flag = true;    
                for (j, q) in p.tower.iter().enumerate() {
                    if !q.finished {
                        flag = false;
                        ret[i].tower[j] = find_program(&q.name, &so_far);
                    }
                }
                if flag {
                    ret[i].finished = true;
                }
            }
        }
    }

    ret
}

fn weight(p: &Program) -> u32 {
    p.tower.iter().fold(p.weight, | acc, x | acc + weight(x))
}

fn balanced(p: &Program) -> bool {
    if p.tower.len() == 0 {
        return true;
    }
    let first = weight(&p.tower[0]);
    for x in &p.tower {
        if weight(&x) != first {
            return false;
        }
    }

    true
}

fn str_to_program(s: &str) -> Program {
    let halves: Vec<_> = s.split(" -> ").collect();
    let tower = if halves.len() < 2 {
        vec![]
    } else {
        halves[1].split(", ").map(|x| new_program(x)).collect()
    };
    
    let pair: Vec<_> = halves[0].split(" ").collect();
    let mut p = new_program(pair[0]);
    p.tower = tower;
    p.weight = pair[1].trim_matches(|x| x == '(' || x == ')').parse().unwrap(); 
    if halves.len() < 2 {
        p.finished = true;
    }

    p
}

fn bottom(programs: &Vec<Program>) -> Program{
    let mut eliminated = std::collections::HashMap::new(); 
    for p in programs {
        for t in &p.tower {
            eliminated.insert(&t.name, true);
        }
    }
    for p in programs {
        if !eliminated.contains_key(&p.name) {
            return p.clone();
        }
    }

    new_program("")
}

fn main() {
    let proj = proj_self::proj_dir(3);
    let file = proj.join("input.txt");
    let input = proj_self::file_to_str(&file);
    let vals = proj_self::str_to_lines(&input);
    let programs = vals.iter().map(|x| str_to_program(x)).collect();
    let b = bottom(&programs);
    println!("Bottom: {}", b.name);
    let weighed = set_all_weights(&programs);
    let unbalanced: Vec<_> = weighed.iter().filter(|x| !balanced(x)).collect();
    println!("Unbalanced nodes: {}", unbalanced.len());
    for x in &unbalanced {
        println!("{} is unbalanced", x.name);
        for p in &x.tower {
            println!("Node: {} Self: {}, Total: {}", p.name, p.weight, weight(&p));
        }
    }
}

#[test]
fn test() {
    let a = str_to_program("pbga (66)");
    assert_eq!(a.name, "pbga");
    assert_eq!(a.weight, 66);
    let b = str_to_program("fwft (72) -> ktlj, cntj, xhth");
    assert_eq!(b.name, "fwft");
    assert_eq!(b.weight, 72);
    assert_eq!(b.tower.len(), 3);
    let sample = vec![
        "pbga (66)",
        "xhth (57)",
        "ebii (61)",
        "havc (66)",
        "ktlj (57)",
        "fwft (72) -> ktlj, cntj, xhth",
        "qoyq (66)",
        "padx (45) -> pbga, havc, qoyq",
        "tknk (41) -> ugml, padx, fwft",
        "jptl (61)",
        "ugml (68) -> gyxo, ebii, jptl",
        "gyxo (61)",
        "cntj (57)",
    ].iter().map(|x| str_to_program(x)).collect();
    let c = bottom(&sample);
    assert_eq!(c.name, "tknk");
    let ws = set_all_weights(&sample);
    let unbalanced: Vec<_> = ws.iter().filter(|x| !balanced(x)).collect();
    assert_eq!(unbalanced.len(), 1);
    assert_eq!(weight(&unbalanced[0].tower[0]), 251);
}
