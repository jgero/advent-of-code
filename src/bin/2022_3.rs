use std::{fs, char};

fn main() {
    let file_content = fs::read_to_string("./rsc/backpack_contents.txt").expect("should have been able to read the file");
    let mut prio_acc = 0;
    for line in file_content.lines() {
        // assume the line has even length, split it in the middle for the 2 stashes
        let stash_1 = line.get(0..( line.len() / 2 )).unwrap();
        let stash_2 = line.get(( line.len() / 2 )..line.len()).unwrap();
        // store values of the letters in bits
        // length 64 is chosen because there are 52 letters
        let mut sto_1: u64 = 0;
        let mut sto_2: u64 = 0;
        for i in 0..stash_1.len() {
            // walk through the letters and put the value of the letter into the store
            sto_1 |= 1 << char_to_index(stash_1.chars().nth(i).unwrap());
            sto_2 |= 1 << char_to_index(stash_2.chars().nth(i).unwrap());
        }
        // find bits that are in both stores
        let mut shared_bit = sto_1 & sto_2;
        loop {
            if shared_bit == 0 {
                break;
            }
            prio_acc += 1;
            shared_bit = shared_bit >> 1;
        }
    }
    println!("final prio is {}", prio_acc);
}

// return the corresponding value to the letters as index
fn char_to_index(c: char) -> usize {
    if c.is_ascii_lowercase() {
        return c as usize - 'a' as usize;
    }
    return c as usize - 'A' as usize + 26;
}
