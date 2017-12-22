
extern crate proj_self;

#[derive(Debug, Clone, Eq, PartialEq)]
enum Tile {
    Blank,
    Vertical,
    Horizontal,
    Junction,
    Letter(String),
}

#[derive(Debug, Clone)]
enum Direction { U, D, L, R }

#[derive(Debug, Clone)]
struct Network {
    tiles: Vec<Vec<Tile>>,
    px: usize,
    py: usize,
    dir: Direction,
    letters: String,
}

fn traverse(network: &Network) -> (String, u32) {
    let mut net = network.clone();
    let mut steps = 1;
    loop {
        let x = net.px;
        let y = net.py;

        //println!("{}, {} => {:?}", x, y, net.dir);

        tick(&mut net);

        if x == net.px && y == net.py {
            break;
        }

        steps += 1;

        match net.tiles[net.py][net.px] {
            Tile::Letter(ref l) => {
                net.letters.push_str(&l);
            }
            _ => (),
        }
    };

    (net.letters, steps)
}

fn is_horizontal(net: &Network, x: usize, y: usize) -> bool {
    match net.tiles[y][x] {
        Tile::Horizontal | Tile::Junction => true,
        Tile::Letter(ref _l) => true,
        _ => false,
    }
}

fn is_vertical(net: &Network, x: usize, y: usize) -> bool {
    match net.tiles[y][x] {
        Tile::Vertical | Tile::Junction => true,
        Tile::Letter(ref _l) => true,
        _ => false,
    }
}

fn not_blank(network: &Network, x: usize, y: usize) -> bool {
    match network.tiles[y][x] {
        Tile::Blank => false,
        _ => true,
    }
}

fn tick(network: &mut Network) {
    let height = network.tiles.len();
    let width = network.tiles[0].len();
    match network.tiles[network.py][network.px] {
        Tile::Junction => {
            match network.dir {
                Direction::U | Direction::D => {
                    if network.px > 0 && is_horizontal(network, network.px - 1, network.py) {
                        network.px -= 1;
                        network.dir = Direction::L;
                    } else if network.px < width - 1 && is_horizontal(network, network.px + 1, network.py) {
                        network.px += 1;
                        network.dir = Direction::R;
                    }
                },
                Direction::L | Direction::R => {
                    if network.py > 0 && is_vertical(network, network.px, network.py - 1) {
                        network.py -= 1;
                        network.dir = Direction::U;
                    } else if network.py < height - 1 && is_vertical(network, network.px, network.py + 1) {
                        network.py += 1;
                        network.dir = Direction::D;
                    }
                }
            }
        }
        _ => {
            match network.dir {
                Direction::U => if network.py > 0 && not_blank(network, network.px, network.py - 1) {
                    network.py -= 1;
                },
                Direction::D => if network.py < height - 1 && not_blank(network, network.px, network.py + 1) {
                    network.py += 1;
                },
                Direction::L => if network.px > 0 && not_blank(network, network.px - 1, network.py) {
                    network.px -= 1;
                },
                Direction::R => if network.px < width - 1 && not_blank(network, network.px + 1, network.py) {
                    network.px += 1;
                },
            }

        }
    }
}

fn line_to_tiles(line: &str) -> Vec<Tile> {
    line.chars().map( |x| {
        match x {
            '|' => Tile::Vertical,
            '-' => Tile::Horizontal,
            '+' => Tile::Junction,
            'a' ... 'z' => Tile::Letter(x.to_string()),
            'A' ... 'Z' => Tile::Letter(x.to_string()),
            _ => Tile::Blank,
        }
    }).collect()
}

fn main() {
    let proj = proj_self::proj_dir(3);
    let file = proj.join("input.txt");
    let input = proj_self::file_to_str(&file);
    let lines: Vec<_> = input.split("\n").collect();
    let tiles: Vec<Vec<Tile>> = lines.iter().map(|x| line_to_tiles(x)).collect();
    let world = Network {
        tiles: tiles.clone(),
        px: 109,
        py: 0,
        dir: Direction::D,
        letters: "".to_string(),
    };
    let (result, steps) = traverse(&world);
    println!("Letters: {}", result);
    println!("Steps: {}", steps);
}

#[test]
fn test() {
    let lines = vec![
        "    |         ",
        "    |  +--+   ",
        "    A  |  C   ",
        "F---|----E|--+",
        "    |  |  |  D",
        "    +B-+  +--+",
    ];
    let tiles: Vec<Vec<Tile>> = lines.iter().map(|x| line_to_tiles(x)).collect();
    let world = Network {
        tiles: tiles.clone(),
        px: 4,
        py: 0,
        dir: Direction::D,
        letters: "".to_string(),
    };
    let (result, steps) = traverse(&world);
    assert_eq!(result, "ABCDEF");
    assert_eq!(steps, 38);
}
