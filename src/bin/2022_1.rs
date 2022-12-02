use std::{fs, str::Lines};

fn main() {
    let file_content = fs::read_to_string("./rsc/calorie_list.txt").expect("should have been able to read the file");
    let mut result: Vec<usize> = Vec::new();
    for cal_per_elf in file_content.split("\n\n") {
        result.push(lines_to_calories(cal_per_elf.lines()));
    }
    result.sort();
    println!("the elf carrying the most calories carries {}",result.last().unwrap());
}

fn lines_to_calories(mut lines: Lines) -> usize {
    let mut calories = 0;
    loop {
        let line = lines.next();
        match line {
            Some(text) => calories += text.parse::<usize>().unwrap(),
            None => break
        }
    }
    return calories;
}
