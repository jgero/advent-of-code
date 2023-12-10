use std::fs;

fn main () {
    let input = fs::read_to_string("rsc/2023_09.txt").unwrap();
    let res: isize = input.lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.split(" ").map(|s| s.parse().unwrap())
             .collect::<Vec<isize>>())
        .map(|nums| nums.last().unwrap() + get_prediction(&nums))
        .reduce(|a,b| a+b)
        .unwrap();
    println!("{:?}", res);
    let res: isize = input.lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.split(" ").map(|s| s.parse().unwrap())
             .collect::<Vec<isize>>())
        .map(|nums| nums.first().unwrap() - get_prediction_back(&nums))
        .reduce(|a,b| a+b)
        .unwrap();
    println!("{:?}", res);
}

fn get_prediction_back(input: &Vec<isize>) -> isize {
    let mut it = input.iter().enumerate();
    it.next();
    let mut it = it.step_by(1);
    let mut diffs: Vec<isize> = Vec::new();
    while let Some((idx, num)) = it.next() {
        diffs.push(num - input.get(idx - 1).unwrap());
    }
    let first = diffs.first().unwrap().clone();
    let guess = if diffs.iter().all(|num| num == &0isize) {
        0isize
    } else {
        first - get_prediction_back(&diffs)
    };
    println!("diffs {:?}", diffs);
    println!("guess {:?}", guess);
    guess
}

fn get_prediction(input: &Vec<isize>) -> isize {
    let mut it = input.iter().enumerate();
    it.next();
    let mut it = it.step_by(1);
    let mut diffs: Vec<isize> = Vec::new();
    while let Some((idx, num)) = it.next() {
        diffs.push(num - input.get(idx - 1).unwrap());
    }
    let last = diffs.last().unwrap().clone();
    if last == 0isize {
        return 0;
    } else {
        return last + get_prediction(&diffs);
    }
}
