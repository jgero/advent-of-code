use std::{fs, collections::HashMap};

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref MAPPING_EXP: Regex = Regex::new(r"([A-Z0-9]{3}) = \(([A-Z0-9]{3}), ([A-Z0-9]{3})\)").unwrap();
}

fn main() {
    let input: Vec<String> = fs::read_to_string("rsc/2023_08.txt").unwrap()
        .split("\n\n")
        .map(|s| String::from(s))
        .collect();
    let instructions = input.get(0).unwrap();
    let map: HashMap<String, Node> = input.get(1).unwrap()
        .lines()
        .map(|l| Node::new(l))
        .map(|n| (n.val.clone(), n))
        .collect();

    let states: Vec<&Node> = map.values().filter(|n| n.is_starting).collect();

    let a: u128 = states.iter().map(|node| {
        let mut iterations = 0;
        let mut next: &Node = node;
        loop {
            if next.is_ending {
                break;
            }
            for c in instructions.chars() {
                match c {
                    'L' => {
                        next = map.get(&next.l).unwrap()
                    },
                    'R' => {
                        next = map.get(&next.r).unwrap()
                    },
                    _ => panic!()
                }
                iterations += 1;
            }
        }
        iterations
    }).reduce(|a,b| num::integer::lcm(a, b)).unwrap();
    println!("{:?}", a);
}

#[derive(Debug, Clone)]
struct Node {
    val: String,
    l: String,
    r: String,
    is_starting: bool,
    is_ending: bool,
}

impl Node {
    fn new(line: &str) -> Self {
        let caps = MAPPING_EXP.captures(line).unwrap();
        let val = caps.get(1).unwrap().as_str().to_string();
        Node {
            val: val.clone(), 
            l:caps.get(2).unwrap().as_str().to_string(),
            r: caps.get(3).unwrap().as_str().to_string(),
            is_starting: val.ends_with("A"),
            is_ending: val.ends_with("Z"),
        }
    }
}
