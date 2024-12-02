use std::collections::HashMap;

const INPUT: &str = include_str!("../../rsc/2024_01.txt");

fn main() {
    let mut row0: Vec<usize> = Vec::new();
    let mut row1: Vec<usize> = Vec::new();
    INPUT.lines()
        .map(|line| line.split(' ')
             .filter(|s|!s.is_empty())
             .map(|s| s.trim())
             .map(|s| s.parse::<usize>().unwrap())
             .collect::<Vec<usize>>()
        )
        .for_each(|row| {
             row0.push(row[0]);
             row1.push(row[1])
        });
    part_1(row0.clone(), row1.clone());
    part_2(row0.clone(), row1.clone());
}

fn part_2(row0: Vec<usize>, row1: Vec<usize>) {
    let mut occurances: HashMap<usize, usize> = HashMap::new();
    for row in row1 {
        occurances.entry(row).and_modify(|e| *e += 1).or_insert(1);
    }
    let mut res = 0;
    for i in row0 {
        if occurances.contains_key(&i) {
            res += i * occurances[&i];
        }
    }
    println!("part 2: {:?}", res);
}

fn part_1(mut row0: Vec<usize>, mut row1: Vec<usize>) {
    row0.sort_unstable();
    row1.sort_unstable();
    let res = row0.iter()
        .zip(row1.iter())
        .map(|p| p.0.abs_diff(*p.1))
        .reduce(|a, b| a + b);
    println!("part 1: {:?}", res.unwrap());
}
