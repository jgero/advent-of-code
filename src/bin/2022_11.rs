use std::{fs, collections::{HashMap, VecDeque}};

use regex::Regex;

fn main() {
    let file_content = fs::read_to_string("./rsc/monkey_business.txt").expect("should have been able to read the file");
    let mut b = MonkeyBusiness::new(&file_content);
    for _ in 0..20 {
        b.do_round();
    }
    println!("the two highest inspection counts added are: {}", b.get_monkey_business());
}

struct MonkeyBusiness {
    // hash map is expected to contain continuous usize indexes as keys
    monkeys: HashMap<usize, Monkey>,
}

impl MonkeyBusiness {
    pub fn new(input: &str) -> MonkeyBusiness {
        let mut monkeys: HashMap<usize, Monkey> = HashMap::new();
        for monkey_string in input.split("\n\n") {
            let m = Monkey::new(monkey_string);
            monkeys.insert(m.index, m);
        }
        MonkeyBusiness { monkeys }
    }

    pub fn do_round(&mut self) {
        // go through all monkeys and assume the indexes in the map are correct
        for i in 0..self.monkeys.len() {
            loop {
                let res = self.monkeys.get_mut(&i).unwrap().try_throw();
                match res {
                    // loop until the monkey cannot throw anything anymore
                    None => break,
                    // the other monkey has to catch
                    Some((index, val)) => self.monkeys.get_mut(&index).unwrap().catch(val)
                }
            }
        }
    }

    // monkey business is the amount of inspections of the two most inspecting monkeys multiplied
    pub fn get_monkey_business(&self) -> usize {
        let mut inspections = Vec::new();
        for ( _, v ) in self.monkeys.iter() {
            inspections.push(v.inspected_count);
        }
        inspections.sort();
        inspections.reverse();
        inspections.get(0).unwrap() * inspections.get(1).unwrap()
    }
}

struct Monkey {
    index: usize,
    items: VecDeque<usize>,
    inspected_count: usize,
    op: Operation,
    test_div_by: usize,
    true_target: usize,
    false_target: usize
}

impl Monkey {
    pub fn new(input: &str) -> Monkey {
        // regex to capture monkey parameters
        // descriptions for capure groups:
        // 1: index
        // 2: starting items
        // 4: operator
        // 5: operator value
        // 6: division test value
        // 7: target index true
        // 8: target index false
        let monkey_regex = Regex::new(r"Monkey (\d):\n  Starting items: (\d+(, \d+)*)\n  Operation: new = old ([+*]) (\d+|old)\n  Test: divisible by (\d*)\n    If true: throw to monkey (\d)\n    If false: throw to monkey (\d)").unwrap();
        let caps = monkey_regex.captures(input).unwrap();
        return Monkey { 
            index: caps.get(1).unwrap().as_str().parse().unwrap(),
            items: usize_list_from_string(caps.get(2).unwrap().as_str()),
            inspected_count: 0,
            op: Operation::from_string(caps.get(4).unwrap().as_str(), caps.get(5).unwrap().as_str()),
            test_div_by: caps.get(6).unwrap().as_str().parse().unwrap(),
            true_target: caps.get(7).unwrap().as_str().parse().unwrap(),
            false_target: caps.get(8).unwrap().as_str().parse().unwrap()
        }
    }

    // try to throw the next item. if the monkey still has an item to throw it will return the
    // target and the item
    pub fn try_throw(&mut self) -> Option<(usize, usize)> {
        let item = self.items.pop_front();
        if item == None {
            return None;
        }
        self.inspected_count += 1;
        let mut item = match self.op {
            Operation::POW => item.unwrap() * item.unwrap(),
            Operation::MULTIPLY(val) => item.unwrap() * val,
            Operation::ADD(val) => item.unwrap() + val
        };
        item = item / 3;
        if item % self.test_div_by == 0 {
            Some((self.true_target, item))
        } else {
            Some((self.false_target, item))
        }
    }

    pub fn catch(&mut self, item: usize) {
        self.items.push_back(item);
    }
}

fn usize_list_from_string(input: &str) -> VecDeque<usize> {
    let mut r = VecDeque::new();
    for val in input.split(", ") {
        r.push_back(val.parse().unwrap());
    }
    return r;
}

enum Operation {
    MULTIPLY(usize),
    ADD(usize),
    POW
}

impl Operation {
    pub fn from_string(op: &str, val: &str) -> Operation {
        match op {
            "*" => {
                if val == "old" {
                    Operation::POW
                } else {
                    Operation::MULTIPLY(val.parse().unwrap())
                }
            },
            "+" => Operation::ADD(val.parse().unwrap()),
            _ => panic!("unknown operator")
        }
    }
}

