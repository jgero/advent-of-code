use std::collections::HashSet;

use itertools::Itertools;

const INPUT: &str = include_str!("../../rsc/2023_11.txt");

fn main() {
    let map = parse_map(INPUT);
    // println!("{:?}", map);
    let distances = map.iter().tuple_combinations()
        .map(|(a,b)|a.get_distance(b))
        .reduce(|a,b| a+b)
        .unwrap();
    println!("{:?}", distances);
}

#[derive(Debug,PartialEq)]
struct Coord {
    x: usize,
    y: usize
}

#[derive(Debug, PartialEq)]
struct Galaxy {
    pos: Coord
}

impl Galaxy {
    fn get_distance(&self, other: &Self) -> usize {
        let dis = ( self.pos.x.max(other.pos.x) - self.pos.x.min(other.pos.x))
            + (self.pos.y.max(other.pos.y) - self.pos.y.min(other.pos.y));
        // println!("dis between {:?} and {:?} is {}", self, other, dis);
        dis
    }
}

fn parse_map(input: &str) -> Vec<Galaxy> {
    let mut empty_y_values: HashSet<usize> = input.lines()
        .enumerate()
        .map(|(i,_)|i)
        .collect();
    let mut empty_x_values: HashSet<usize> = input.lines()
        .take(1)
        .map(|line| line.chars().enumerate())
        .flatten()
        .map(|(i,_)|i)
        .collect();
    let mut res: Vec<Galaxy> = input.lines()
        .enumerate()
        .map(|(y, line)| line.chars().enumerate().map(move |(x, c)| ((x,y),c)))
        .flatten()
        .filter_map(|((x,y),c)| if c == '#' { Some(Galaxy{pos: Coord { x, y }}) } else { None })
        .collect();
    println!("{:?}", res);
    for g in &mut res {
        empty_x_values.remove(&g.pos.x);
        empty_y_values.remove(&g.pos.y);
    }
    for g in &mut res {
        g.pos.x += empty_x_values.iter().filter(|it| *it < &g.pos.x).count() * 999999;
        g.pos.y += empty_y_values.iter().filter(|it| *it < &g.pos.y).count() * 999999;
    }
    res
}
