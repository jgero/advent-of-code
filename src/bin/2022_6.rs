use std::fs;

const WINDOW_SIZE: usize = 4;

fn main() {
    let file_content = fs::read_to_string("./rsc/signal_streams.txt").expect("should have been able to read the file");
    for line in file_content.lines() {
        match find_marker(line) {
            None => panic!("no start of packet in line {}", line),
            Some(i) => println!("start of packet is {} for line {}", i + WINDOW_SIZE, line)
        }
    }
}

fn find_marker(input: &str) -> Option<usize> {
    // look at bytes in windows of 4
    input.as_bytes().windows(WINDOW_SIZE)
        // return position of the window
        .position(|window_elements| {
            // used to compare occurrences of the letters
            let mut checker: u32 = 0;
            // go through all bytes in the window
            for el in window_elements {
                let reference = checker;
                // 1. subtract 'a' as byte from byte value of the element. this enumerates all
                // lowercase letters staring with 0 for 'a'.
                // 2. shift the 1 to the left by the index of the letter
                // 3. use OR to add the current letter to the checker
                checker |= 1 << (el - b'a');
                // if the checker has not changed no bit was set,
                // which means the letter wasn't new
                if checker == reference {
                    return false;
                }
            }
            return true;
        })
}
