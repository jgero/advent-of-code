use std::fs;

use regex::Regex;

fn main() {
    let file_content = fs::read_to_string("./rsc/cpu_instructions.txt").expect("should have been able to read the file");
    let mut exec = ProgramExecutor::new(Instruction::from_string(file_content));
    let interesting_cycles = vec![20, 60, 100, 140, 180, 220];
    let mut strengths = 0;
    for c in interesting_cycles {
        strengths += exec.cycle_to(c);
    }
    println!("accumulated strengths are: {}", strengths);
}

struct ProgramExecutor {
    instructions: Vec<Instruction>,
    // contains the cycle number the executor is currently in
    cycle: usize,
    // write operations take 2 cycles to complete. in the first cycle the operation is read and the
    // value of the addx is written into the buffer. if the buffer is not empty at the start of a
    // cycle the value has to be handled. if the buffer is empty a new instruction can be read
    buffer: Option<i16>,
    register: i16
}

impl ProgramExecutor {
    pub fn new(instructions: Vec<Instruction>) -> ProgramExecutor {
        ProgramExecutor { instructions, cycle: 1, buffer: None, register: 1 }
    }

    // runs from current cycle to provided cycle and returns the signal strength
    pub fn cycle_to(&mut self, cycle: usize) -> i16 {
        // + 1 to include the target cycle
        for i in self.cycle + 1..cycle + 1 {
            self.cycle = i;
            match self.buffer {
                // do next instruction if buffer is empty
                None => {
                    match self.instructions.pop().unwrap() {
                        Instruction::NOOP => {},
                        Instruction::ADDX(val) => {
                            self.buffer = Some(val);
                        }
                    }
                },
                // add buffer to register and do not pop next instruction
                Some(val) => {
                    self.buffer = None;
                    self.register += val;
                }
            };
        }
        return i16::try_from(self.cycle).unwrap() * self.register;
    }
}

enum Instruction {
    NOOP,
    ADDX(i16)
}

impl Instruction {
    // reads the instructions from a String.
    // reverses the order so "pop" can be used to read the instructions from top to bottom
    pub fn from_string(lines: String) -> Vec<Instruction> {
        let mut res = Vec::new();
        let add_regex = Regex::new(r"^addx (-?\d+)$").unwrap();
        for line in lines.lines() {
            if line.contains("noop") {
                res.push(Instruction::NOOP);
            } else {
                let caps = add_regex.captures(line).unwrap();
                res.push( Instruction::ADDX(caps.get(1).unwrap().as_str().parse().unwrap()) );
            }
        }
        res.reverse();
        return res;
    }
}
