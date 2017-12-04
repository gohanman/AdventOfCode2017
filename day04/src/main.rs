
extern crate proj_self;

fn line_to_words(input: &str) -> Vec<String> {
    let as_string = input.to_string();
    let iter = as_string.split(' ');
    iter.map(|x| x.trim().to_string())
        .collect()
}

fn is_equal(a: &str, b: &str) -> bool {
    a == b
}

fn is_anagram(a: &str, b: &str) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut a_chars: Vec<char> = a.chars().collect();
    let mut b_chars: Vec<char> = b.chars().collect();
    a_chars.sort_by(|x, y| x.cmp(y));
    b_chars.sort_by(|x, y| x.cmp(y));

    a_chars == b_chars
}

fn validate(input: &Vec<String>, func: &Fn(&str, &str) -> bool) -> bool{
    let mut i = 0;
    while i < (input.len() - 1) {
        let mut j = i+1;
        while j < input.len() {
            if func(&input[i], &input[j]) {
                return false;
            }
            j += 1;
        }
        i += 1;
    }

    true
}

fn main() {
    let proj = proj_self::proj_dir(3);
    let file = proj.join("input.txt");
    let input = proj_self::file_to_str(&file);
    let vals = proj_self::str_to_lines(&input);
    let valid: Vec<&&str> = vals.iter().filter(|x| validate(&line_to_words(x), &is_equal)).collect();
    println!("Valid: {}", valid.len());
    let also_valid: Vec<&&str> = vals.iter().filter(|x| validate(&line_to_words(x), &is_anagram)).collect();
    println!("Anagram Valid: {}", also_valid.len());
}

#[test]
fn test() {
    let a = line_to_words("aa bb cc dd ee");
    assert_eq!(true, validate(&a, &is_equal));
    let b = line_to_words("aa bb cc dd aa");
    assert_eq!(false, validate(&b, &is_equal));
    let c = line_to_words("aa bb cc dd aaa");
    assert_eq!(true, validate(&c, &is_equal));
    let d = line_to_words("abcde fghij");
    assert_eq!(true, validate(&d, &is_anagram));
    let e = line_to_words("abcde xyz ecdab");
    assert_eq!(false, validate(&e, &is_anagram));
    let f = line_to_words("a ab abc abd abf abj");
    assert_eq!(true, validate(&f, &is_anagram));
    let g = line_to_words("iiii oiii ooii oooi oooo");
    assert_eq!(true, validate(&g, &is_anagram));
    let h = line_to_words("oiii ioii iioi iiio");
    assert_eq!(false, validate(&h, &is_anagram));
}
