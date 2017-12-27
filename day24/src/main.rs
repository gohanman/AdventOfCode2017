
extern crate proj_self;

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
struct Component {
    p1: usize,
    link1: bool,
    p2: usize,
    link2: bool,
}
impl Component {
    pub fn from_str(s: &str) -> Component {
        let parts: Vec<_> = s.trim().split('/').collect();
        Component { p1: parts[0].parse().unwrap(), p2: parts[1].parse().unwrap(), link1: false, link2: false }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Bridge {
    spans: Vec<Component>,
    spares: std::collections::HashMap<Component,u8>,
}
impl Bridge {
    pub fn new(comps: &Vec<Component>) -> Bridge {
        let mut hm = std::collections::HashMap::new();
        for c in comps.iter() {
            hm.insert(c.clone(), 1);
        }

        Bridge { spans: vec![], spares: hm }
    }

    pub fn need(&self) -> usize {
        match self.spans.last() {
            Some(c) => if c.link1 { c.p2 } else { c.p1 },
            None => 0,
        }
    }

    pub fn fits(&self, c: &Component) -> bool {
        let n = self.need();
        if !c.link1 && c.p1 == n {
            true
        } else if !c.link2 && c.p2 == n {
            true
        } else {
            false
        }
    }

    pub fn add(&self, c: &Component) -> Bridge {
        let mut b = self.clone();
        let mut cmp = c.clone();
        let n = b.need();
        if n == cmp.p1 {
            cmp.link1 = true;
        } else {
            cmp.link2 = true;
        }
        b.spans.push(cmp);
        b.spares.remove(c);

        b
    }

    pub fn strength(&self) -> usize {
        let mut ttl = 0;
        for c in self.spans.iter() {
            ttl += c.p1 + c.p2;
        }

        ttl
    }

    pub fn print(&self) {
        for c in self.spans.iter() {
            print!("{}/{}--", c.p1, c.p2); 
        }
        println!(" ({}) <{}>", self.spares.len(), self.need());
    }
}

fn all_bridges(components: &Vec<Component>) -> Vec<Bridge> {
    let mut bridges = vec![];
    bridges.push(Bridge::new(components));
    let mut pos = 0;
    loop {
        /*
        println!("------------ {} -----------", pos);
        for (i, dbg) in bridges.iter().enumerate() {
            print!("{}: ", i);
            dbg.print();
        }
        */
        let b = bridges[pos].clone();
        let available: Vec<&Component> = b.spares.keys().filter(|x| b.fits(x)).collect();
        if available.len() == 0 {
            pos += 1;
        } else {
            bridges[pos] = b.add(available[0]);
            for i in 1..available.len() {
                bridges.push(b.add(available[i]));
            }
        }
        if pos >= bridges.len() {
            break;
        }
    }

    bridges
}

fn strongest(bridges: &Vec<Bridge>) -> usize {
    let mut max = 0;
    for b in bridges {
        let str = b.strength();
        if str > max {
            max = str;
        }
    }

    max
}

fn longest(bridges: &Vec<Bridge>) -> Vec<Bridge> {
    let mut max = 0;
    let mut ret = vec![];
    for b in bridges.iter() {
        let len = b.spans.len();
        if len > max {
            max = len;
            ret = vec![];
            ret.push(b.clone());
        } else if len == max {
            ret.push(b.clone());
        }
    }

    ret
}

fn main() {
    let proj = proj_self::proj_dir(3);
    let file = proj.join("input.txt");
    let input = proj_self::file_to_str(&file);
    let lines = proj_self::str_to_lines(&input);

    let comp: Vec<Component> = lines.iter().map(|x| Component::from_str(x)).collect();
    let all = all_bridges(&comp);
    println!("{} options, strongest is {}", all.len(), strongest(&all));

    let long = longest(&all);
    println!("Longest ({}), Strongest ({})", long[0].spans.len(), strongest(&long));
}

#[test]
fn test() {
    let lines = vec![
        "0/2",
        "2/2",
        "2/3",
        "3/4",
        "3/5",
        "0/1",
        "10/1",
        "9/10",
    ];
    let comp: Vec<Component> = lines.iter().map(|x| Component::from_str(x)).collect();
    let all = all_bridges(&comp);
    assert_eq!(strongest(&all), 31);

    let long = longest(&all);
    assert_eq!(strongest(&long), 19);
}
