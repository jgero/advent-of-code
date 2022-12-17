use std::fs;

use regex::Regex;

fn main() {
    let file_content = fs::read_to_string("./rsc/command_outputs.txt").expect("should have been able to read the file");
    let mut dirs: Vec<Directory> = Vec::new();
    let mut current_dir = 0;
    for line in file_content.lines() {
        current_dir = parse_line(line, &mut dirs, current_dir);
    }

    let mut acc = 0;
    for dir in dirs.iter() {
        if dir.size <= 100000 {
            acc += dir.size;
        }
    }
    println!("accumulated size of all dirs smaller than 100000 is {}", acc);
}

struct Directory {
    name: String,
    parent: Option<usize>,
    subdirs:Vec<usize>,
    files: Vec<File>,
    size: usize
}

impl Directory {
    pub fn new(name: &str, parent: Option<usize>) -> Directory {
        return Directory { name: String::from(name), parent, subdirs: Vec::new(), files: Vec::new(), size: 0}
    }

}

struct File {
    name: String,
    size: usize
}

impl File {
    pub fn new(name: &str, size: &str) -> File {
        File { name: String::from(name), size: size.parse().unwrap() }
    }
}


fn parse_line(line: &str, dirs:&mut Vec<Directory>, current_dir: usize) -> usize {
    let file_regex: Regex = Regex::new(r"^(\d+) ([a-z.]+)$").unwrap();
    let cd_regex: Regex = Regex::new(r"^\$ cd ([a-z/.]+)$").unwrap();
    let ls_regex: Regex = Regex::new(r"^\$ ls$").unwrap();
    let dir_regex: Regex = Regex::new(r"dir ([a-z]+)$").unwrap();
    if file_regex.is_match(line) {
        let caps = file_regex.captures(line).unwrap();
        dirs.get_mut(current_dir).unwrap().files.push(
            File::new(caps.get(2).unwrap().as_str(),caps.get(1).unwrap().as_str()));
        return current_dir;
    } else if cd_regex.is_match(line) {
        let caps = cd_regex.captures(line).unwrap();
        let sw_dir = caps.get(1).unwrap().as_str();
        if sw_dir == "/" {
            dirs.push(Directory::new("/", None));
            return 0;
        } else if sw_dir == ".." {
            let mut acc = 0;
            // let mut curr = dirs.get_mut(current_dir).unwrap();
            for f in dirs.get_mut(current_dir).unwrap().files.iter() {
                acc += f.size;
            }
            for d in dirs.get(current_dir).unwrap().subdirs.iter() {
                acc += dirs.get(*d).unwrap().size;
            }
            dirs.get_mut(current_dir).unwrap().size = acc;
            return dirs.get_mut(current_dir).unwrap().parent.unwrap();
        } else {
            return *dirs.get(current_dir).unwrap().subdirs.iter().find(|&subdir| dirs.get(*subdir).unwrap().name == sw_dir).unwrap();
        }
    } else if ls_regex.is_match(line) {
        return current_dir;
    } else if dir_regex.is_match(line) {
        let caps = dir_regex.captures(line).unwrap();
        dirs.push(Directory::new(caps.get(1).unwrap().as_str(), Some(current_dir)));
        let new_dir_ind = dirs.len() - 1;
        dirs.get_mut(current_dir).unwrap().subdirs.push(new_dir_ind);
        return current_dir;
    } else {
        panic!("could not parse line: {}", line);
    }
}
