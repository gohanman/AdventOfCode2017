
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Bank {
    blocks: u32,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct World {
    banks: Vec<Bank>,
}

fn redistribute(w: &World, bank: usize) -> World{
    let mut new_banks = w.banks.clone();
    let mut pos = bank;
    let mut available = new_banks[pos].blocks;
    new_banks[pos].blocks = 0;
    while available > 0 {
        pos = (pos + 1) % new_banks.len();
        new_banks[pos].blocks += 1;
        available -= 1;
    }

    World { banks: new_banks }
}

fn fullest(w: &World) -> usize {
    let mut max = 0;
    let mut max_i = 0;
    for i in 0..w.banks.len() {
        if w.banks[i].blocks > max {
            max = w.banks[i].blocks;
            max_i = i;
        }
    }

    max_i
}

fn repeats(w: &World) -> (u64, u64) {
    let mut cycles = 0;
    let mut states = std::collections::HashMap::new();
    let mut state = World { banks: w.banks.clone() };
    loop {
        states.insert(state.clone(), cycles);
        state = redistribute(&state, fullest(&state));
        cycles += 1;
        if states.contains_key(&state) {
            break;
        }
    }

    (cycles, cycles - states[&state])
}

fn main() {
    let init = vec![
        Bank { blocks: 14 },
        Bank { blocks: 0 },
        Bank { blocks: 15 },
        Bank { blocks: 12 },
        Bank { blocks: 11 },
        Bank { blocks: 11 },
        Bank { blocks: 3 },
        Bank { blocks: 5 },
        Bank { blocks: 1 },
        Bank { blocks: 6 },
        Bank { blocks: 8 },
        Bank { blocks: 4 },
        Bank { blocks: 9 },
        Bank { blocks: 1 },
        Bank { blocks: 8 },
        Bank { blocks: 4 },
    ];
    let w = World { banks: init };
    let (cycle, len) = repeats(&w);
    println!("Cycled at {}", cycle);
    println!("Cycled length {}", len);
}

#[test]
fn test() {
    let banks = vec![
        Bank { blocks: 0 },
        Bank { blocks: 2 },
        Bank { blocks: 7 },
        Bank { blocks: 0 },
    ];
    let w = World { banks: banks };
    let (a, b) = repeats(&w);
    assert_eq!(5, a);
    assert_eq!(4, b);
}
