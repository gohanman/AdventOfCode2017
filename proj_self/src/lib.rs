
use std::path::PathBuf;
use std::io::Read;

// remove [depth] entries from the path to the current
// executable and return the resulting path
pub fn proj_dir(depth: u32) -> PathBuf {

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

// read a file into a string and return it
pub fn file_to_str(file: &PathBuf) -> String {
    let mut input = String::new();
    let _io = std::fs::File::open(file).unwrap().read_to_string(
        &mut input,
    );
    input
}

// split a string into lines
pub fn str_to_lines(input: &str) -> Vec<&str> {
    input
        .split("\n")
        .map(|x| x.trim())
        .filter(|x| x.len() > 0)
        .collect()
}

#[cfg(test)]
mod tests {
    use std;
    use proj_dir;
    use file_to_str;

    #[test]
    fn it_works() {
        let me = std::env::current_exe().unwrap();
        let me2 = proj_dir(0);
        assert_eq!(me, me2);

        let my_parent = me.parent().unwrap().to_path_buf();
        let my_parent2 = proj_dir(1);
        assert_eq!(my_parent, my_parent2);

        let mut path = proj_dir(4);
        path.push("src");
        path.push("lib.rs");
        let my_code = file_to_str(&path);
        assert!(my_code.len() > 0, "Couldn't find myself");
    }
}
