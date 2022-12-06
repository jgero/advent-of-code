use std::{fs, char};
use regex::Regex;

type Crate = char;
type Stack = Vec<Crate>;
type CrateYard = Vec<Stack>;

const CRATE_WIDTH: usize = 3;
const EMPTY_CRATE: &str = "   ";

#[derive(Debug)]
struct MoveInstruction {
    from: usize,
    to: usize,
    amount: usize
}

impl MoveInstruction {
    pub fn from_str(input: &str) -> MoveInstruction {
        let re = Regex::new(r"move (\d) from (\d) to (\d)").unwrap();
        let m = re.captures(input).unwrap();
        return MoveInstruction{
            from: m.get(2).unwrap().as_str().parse().unwrap(),
            to: m.get(3).unwrap().as_str().parse().unwrap(),
            amount: m.get(1).unwrap().as_str().parse().unwrap()
        };
    }
}

fn main() {
    let file_content = fs::read_to_string("./rsc/cargo_crane_instructions.txt").expect("should have been able to read the file");
    let content_split: Vec<&str> = file_content.split("\n\n").collect();
    let mut yard = parse_content_to_crate_yard(content_split[0]);
    let instructions = parse_instructions(content_split[1]);
    run_instructions(&instructions, &mut yard);
    for (i, val) in yard.iter().enumerate() {
        println!("stack {} has crate {} on top", i + 1, val.last().unwrap());
    }
}

fn run_instructions(instructions: &Vec<MoveInstruction>, yard: &mut CrateYard) {
    for instr in instructions {
        for _ in 0..instr.amount {
            let el = yard[instr.from - 1].pop().unwrap();
            yard[instr.to - 1].push(el);
        }
    }
}

fn parse_instructions(input: &str) -> Vec<MoveInstruction> {
    let mut val = Vec::new();
    for line in input.lines() {
        val.push(MoveInstruction::from_str(line));
    }
    return val;
}

fn parse_content_to_crate_yard(input: &str) -> CrateYard {
    let lines: Vec<&str> = input.lines().collect();
    // add 1 to line lengt to equalize calculating separator for every crate
    let stack_count = (lines.get(0).unwrap().len() + 1) / (CRATE_WIDTH + 1);
    let mut yard = vec![Vec::new(); stack_count];
    // skip the last line because it only contains the numbers
    for line_no in 0..lines.len() - 1 {
        parse_crate_line_to_resources(lines.get(line_no).unwrap(), &mut yard, stack_count);
    }
    return yard;
}

// expect a string in the form of:
// - "[a] [r]     [f]"
fn parse_crate_line_to_resources(line: &str, yard: &mut CrateYard, stack_count: usize) {
    for crate_index in 0..stack_count {
        match crate_string_to_u8(&line[crate_index * CRATE_WIDTH + crate_index..(crate_index + 1) * CRATE_WIDTH + crate_index]) {
            Some(val) => yard[crate_index].insert(0, val),
            None => ()
        }
    }
}

// input possibilities:
// - only whitespaces (no crate)
// - char surrounded by square brackets: "[A]"
fn crate_string_to_u8(input: &str) -> Option<char> {
    if input == EMPTY_CRATE {
        return None;
    }
    return Some(input.chars().nth(1).unwrap());
}
