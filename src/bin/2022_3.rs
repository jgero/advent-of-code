use std::{fs, char};

fn main() {
    let file_content = fs::read_to_string("./rsc/backpack_contents.txt").expect("should have been able to read the file");
    let mut prio_acc = 0;
    for line in file_content.lines() {
        let stash_1 = line.get(0..( line.len() / 2 )).unwrap();
        let stash_2 = line.get(( line.len() / 2 )..line.len()).unwrap(); 
        let mut char_bin_1:  [bool; 52] = [false; 52];
        let mut char_bin_2:  [bool; 52] = [false; 52];
        for i in 0..stash_1.len() {
            char_bin_1[char_to_index(stash_1.chars().nth(i).unwrap())] = true;
            char_bin_2[char_to_index(stash_2.chars().nth(i).unwrap())] = true;
        }

        for i in ( 0..51 ).rev() {
            if char_bin_1[i] && char_bin_2[i] {
                prio_acc += i + 1;
            }
        }
    }
    println!("final prio is {}", prio_acc);
}

fn char_to_index(c: char) -> usize {
    if c.is_ascii_lowercase() {
        return c as usize - 'a' as usize;
    }
    return c as usize - 'A' as usize + 26;
}
