use regex::Regex;

const INPUT: &str = include_str!("../../rsc/2024_03.txt");

#[derive(Debug)]
enum Instruction {
    Mul(usize, usize),
    Do,
    Dont
}

impl <'a> From<regex::Captures<'a>> for Instruction {
    fn from(value: regex::Captures) -> Self {
        if value.get(4).is_some() {
            return Instruction::Do;
        } else if value.get(5).is_some() {
            return Instruction::Dont;
        } else {
            return Instruction::Mul(value.get(2).unwrap().as_str().parse::<usize>().unwrap(), value.get(3).unwrap().as_str().parse::<usize>().unwrap());
        }
    }
}

fn main() {
    let rx = Regex::new(r"(mul\(([\d]+),([\d]+)\))|(do\(\))|(don't\(\))").unwrap();
    let instructions = rx.captures_iter(INPUT)
        .map(|cap| cap.into())
        .collect::<Vec<Instruction>>();
    let mut enabled = true;
    let mut res = 0;
    for i in instructions {
        match i {
            Instruction::Mul(a,b) => {
                if enabled {
                    res += a * b;
                }
            },
            Instruction::Do => {
                enabled = true;
            },
            Instruction::Dont => {
                enabled = false;
            }
        }
    }
    println!("muls: {:?}", res);
}
