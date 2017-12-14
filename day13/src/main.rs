
extern crate proj_self;

#[derive(Clone,Eq,PartialEq,Debug)]
struct Layer {
    range: i32,
    scanner: i32,
    down: bool,
}

#[derive(Clone,Eq,PartialEq,Debug)]
struct Firewall {
    layers: Vec<Layer>,
    packet: i32,
}

impl Layer {
    pub fn new(r: i32) -> Layer {
        Layer { range: r, scanner: 0, down: true }
    }
}

impl Firewall {
    pub fn build(self: &mut Firewall, layer: Layer, pos: usize) {
        while pos > self.layers.len() {
            self.layers.push(Layer::new(0));
        }
        self.layers.push(layer);
    }
}

fn lines_to_fw(lines: &Vec<&str>) -> Firewall {
    let mut fw = Firewall { layers: vec![], packet: -1 };
    for line in lines.iter() {
        let pair: Vec<i32> = line.split(": ").map(|x| x.trim().parse().unwrap()).collect();
        let layer = Layer::new(pair[1]);
        fw.build(layer, pair[0] as usize);
    }
    fw
}

fn delayed_fw(lines: &Vec<&str>, delay: i32) -> Firewall {
    let mut fw = lines_to_fw(lines);
    fw.packet -= delay;
    fw
}

fn tick(layer: &Layer) -> Layer {
    if layer.range < 2 {
        return layer.clone();
    }
    if layer.down {
        if layer.scanner + 1 > layer.range {
            Layer { range: layer.range, scanner: layer.scanner - 1, down: false }
        } else {
            Layer { range: layer.range, scanner: layer.scanner + 1, down: true }
        }
    } else {
        if layer.scanner <= 1 {
            Layer { range: layer.range, scanner: layer.scanner + 1, down: true }
        } else {
            Layer { range: layer.range, scanner: layer.scanner - 1, down: false }
        }
    }
}

fn is_hit(fw: &Firewall) -> bool {
    if fw.packet < 0 {
        return false;
    }
    let layer = &fw.layers[fw.packet as usize];
    layer.range > 0 && layer.scanner == 1
}

fn damage(fw: &Firewall) -> i32 {
    if fw.packet < 0 {
        return 0;
    }
    let depth = fw.packet;
    let severity = fw.layers[depth as usize].range;

    depth * severity
}

fn is_done(fw: &Firewall) -> bool {
    fw.packet >= fw.layers.len() as i32
}

fn travel(firewall: &Firewall) -> (Firewall, bool) {
    let hit  = is_hit(firewall); 
    let new_layers = firewall.layers.iter().map(|x| tick(x)).collect();
    let new_fw = Firewall { layers: new_layers, packet: firewall.packet + 1 };
    (new_fw, hit)
}

fn count_damage(firewall: &Firewall, or_hits: bool) -> i32 {
    let mut dmg = 0;
    let mut fw = firewall.clone();
    while !is_done(&fw) {
        let d = damage(&fw);
        let (new_fw, hit) = travel(&fw);
        fw = new_fw;
        if hit {
            dmg += d;
            if or_hits {
                return 1;
            }
        }
    }

    dmg
}

fn safe_delay(lines: &Vec<&str>) -> i32 {
    let mut delay = 1;
    loop {
        println!("Try delay: {}", delay);
        let fw = delayed_fw(lines, delay);
        let dmg = count_damage(&fw, true);
        if dmg == 0 {
            return delay;
        }
        delay += 1;
    }
}

fn main() {
    let proj = proj_self::proj_dir(3);
    let file = proj.join("input.txt");
    let input = proj_self::file_to_str(&file);
    let lines = proj_self::str_to_lines(&input);
    let fw = lines_to_fw(&lines);
    let hits = count_damage(&fw, false);
    println!("Damage: {}", hits);
    let delay = safe_delay(&lines);
    println!("Wait time: {}", delay);
}

#[test]
fn test() {
    let lines = vec![
         "0: 3",
         "1: 2",
         "4: 4",
         "6: 4",
    ];
    let fw = lines_to_fw(&lines);
    assert_eq!(7, fw.layers.len());
    let hits = count_damage(&fw, false);
    assert_eq!(24, hits);
    assert_eq!(10, safe_delay(&lines));
}
