
use std::path::PathBuf;
use std::io::Read;

fn proj_dir(depth: u32) -> PathBuf {

    fn rm_dirs(num: u32, path: PathBuf) -> PathBuf {
        if num == 0 {
            return path;
        }

        let parent = path.parent().unwrap().to_path_buf();
        rm_dirs(num - 1, parent)
    }

    let exe = std::env::current_exe().unwrap();
    rm_dirs(depth, exe)
}

fn solve(bytes:&Vec<u8>, step:usize) -> u64 {
    bytes.iter().enumerate().fold(0u64, | acc, (i,val)| {
        let next = bytes[(i + step) % bytes.len()];
        let inc = (*val as u64) - 48;
        if *val == next { acc + inc } else { acc }
    })
}

fn captcha(input:&str) -> u64 {
    let bytes = input.trim().to_string().into_bytes();
    solve(&bytes, 1)
}

fn wide_captcha(input:&str) -> u64 {
    let bytes = input.trim().to_string().into_bytes();
    solve(&bytes, bytes.len() / 2)
}

fn main() {
    let proj = proj_dir(3);
    let file = proj.join("input.txt");
    let mut input = String::new();
    let _io = std::fs::File::open(file).unwrap().read_to_string(
        &mut input,
    );
    println!("In: {}", input);
    let answer = captcha(&input);
    println!("Captcha {}", answer);
    let answer2 = wide_captcha(&input);
    println!("Captcha {}", answer2);
}

#[test]
fn test() {
    let a = captcha("1122");
    assert_eq!(3, a);
    let b = captcha("1111");
    assert_eq!(4, b);
    let c = captcha("1234");
    assert_eq!(0, c);
    let d = captcha("91212129");
    assert_eq!(9, d);
}

