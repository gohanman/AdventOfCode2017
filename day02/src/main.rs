
extern crate proj_self;

fn div_checksum(lines: &Vec<Vec<u64>>) -> u64 {
    lines.iter().fold(0, | acc, x | {
        let mut val = 0;
        for i in x.iter() {
            for j in x.iter() {
                if i != j && i % j == 0 {
                    val = i / j;
                } else if i != j && j % i == 0 {
                    val = j / i;
                }
            }
        }
        acc + val
    })
}

fn checksum(lines: &Vec<Vec<u64>>) -> u64 {
    lines.iter().fold(0, | acc, x | {
        acc + line_checksum(&mut x.clone())
    })
}

fn line_checksum(line: &mut Vec<u64>) -> u64 {
    line.sort();
    let min = line[0];
    let max = line[line.len() - 1];
    max - min
}

fn lines_to_vals(lines: &Vec<&str>) -> Vec<Vec<u64>> {
    lines.iter().map(|x| {
        let nums = x.split_whitespace();
        nums.map(|y| y.parse().unwrap()).collect()
    }).collect()
}

fn str_to_lines(input: &str) -> Vec<&str> {
    input.split("\n").map(|x| x.trim()).filter(|x| x.len() > 0).collect()
}

fn main() {
    let proj = proj_self::proj_dir(3);
    let file = proj.join("input.txt");
    let input = proj_self::file_to_str(&file);
    let vals = lines_to_vals(&str_to_lines(&input));
    let c = checksum(&vals);
    println!("Checksum: {}", c);
    let c2 = div_checksum(&vals);
    println!("Checksum2: {}", c2);
}

#[test]
fn test() {
    let input = "5 1 9 5\r\n7 5 3\r\n2 4 6 8";
    let lines = str_to_lines(input);
    let ints = lines_to_vals(&lines);
    let c = checksum(&ints);
    assert_eq!(18, c);

    let in2 = "5 9 2 8\n9 4 7 3\n3 8 6 5";
    let lines2 = str_to_lines(in2);
    let ints2 = lines_to_vals(&lines2);
    let c2 = div_checksum(&ints2);
    assert_eq!(9, c2);
}
