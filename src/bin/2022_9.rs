use std::{fs, collections::HashSet};

use regex::Regex;

fn main() {
    let file_content = fs::read_to_string("./rsc/series_of_motions.txt").expect("should have been able to read the file");
    let mut instructions = Vec::new();
    for line in file_content.lines() {
        instructions.push(Direction::from(line));
    }
    let mut r = Rope::new();
    let mut s: HashSet<Coord> = HashSet::new();
    for i in instructions {
        for _ in 0..i.1 {
            r.move_head(&i.0);
            s.insert(r.tail);
        }
    }
    println!("number of coords used by tail: {}", s.len());
}

// coordinates start at the bottom left
#[derive(Debug, Hash, Clone, Copy)]
struct Coord {
    x: i8,
    y: i8
}

impl Coord {
    pub fn move_in_direction(&mut self, d: &Direction) {
        match d {
            Direction::UP => self.y += 1,
            Direction::DOWN => self.y -= 1,
            Direction::RIGHT => self.x += 1,
            Direction::LEFT => self.x -= 1
        }
    }
}

impl Eq for Coord {}
impl PartialEq for Coord {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

#[derive(Debug)]
struct Rope {
    head: Coord,
    tail: Coord
}

impl Rope {
    pub fn new() -> Self {
        return Rope { head: Coord { x: 0, y: 0 }, tail: Coord { x: 0, y: 0 } }
    }

    fn fix_tail_x(&mut self) {
        if self.head.x > self.tail.x {
            self.tail.move_in_direction(&Direction::RIGHT);
        } else {
            self.tail.move_in_direction(&Direction::LEFT);
        }
    }

    fn fix_tail_y(&mut self) {
        if self.head.y > self.tail.y {
            self.tail.move_in_direction(&Direction::UP);
        } else {
            self.tail.move_in_direction(&Direction::DOWN);
        }
    }

    pub fn move_head(&mut self, d: &Direction) {
        self.head.move_in_direction(d);
        let offset_x = (self.head.x - self.tail.x).abs();
        let offset_y = (self.head.y - self.tail.y).abs();
        if offset_x <= 1 && offset_y <= 1 {
            // head and tail are close enough, no action on tail necessary
            return;
        }

        if offset_x > 1 {
            self.fix_tail_x();
            if offset_y != 0 {
                self.fix_tail_y();
            }
        }

        if offset_y > 1 {
            self.fix_tail_y();
            if offset_x != 0 {
                self.fix_tail_x();
            }
        }
    }
}

enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT
}

impl Direction {
    pub fn from(input: &str) -> (Self, usize) {
        let r: Regex = Regex::new(r"^([UDLR]) (\d)+$").unwrap();
        let caps = r.captures(input).unwrap();
        let direction = caps.get(1).unwrap().as_str();
        let count = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
        match direction {
            "U" => (Direction::UP, count),
            "D" => (Direction::DOWN, count),
            "L" => (Direction::LEFT, count),
            "R" => (Direction::RIGHT, count),
            _ => panic!("not possible")
        }
    }
}

