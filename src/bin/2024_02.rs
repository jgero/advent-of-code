const INPUT: &str = include_str!("../../rsc/2024_02.txt");

fn main() {
    let valid = INPUT.lines()
        .map(|line| line
             .split(' ')
             .filter(|s|!s.is_empty())
             .map(|s| s.parse::<isize>().unwrap())
             .collect::<Vec<isize>>()
        )
        .map(|instructions| if check(instructions, 0, 0, false) { 1 } else { 0 })
        .reduce(|a,b| a+b);
    println!("{:?}", valid);
}

fn check(instruction: Vec<isize>, index: usize, prev_diff: isize, has_skipped: bool) -> bool {
    if index == instruction.len()-1 {
        return true;
    }

    let diff = instruction[index] - instruction[index+1];

    if diff.abs() < 1 || diff.abs() > 3 || (diff < 0 && prev_diff > 0) || (diff > 0 && prev_diff < 0) {
        for remove in 0..instruction.len() {
            if has_skipped {
                return false;
            } else {
                let skip_level = [&instruction[0..remove], &instruction[remove+1..instruction.len()]].concat();
                if check(skip_level, 0, 0, true) {
                    return true;
                }
            }
        }

        return false;
    }

    return check(instruction, index+1, diff, has_skipped);
}

// fn part_1() {
//     let res = INPUT.lines()
//         .map(|line| line
//              .split(' ')
//              .filter(|s|!s.is_empty())
//              .map(|s| s.parse::<isize>().unwrap())
//              .collect::<Vec<isize>>()
//         )
//         .map(|seq| seq
//              .windows(2)
//              .map(|w| w[0] - w[1])
//              .collect::<Vec<isize>>()
//         )
//         .map(|seq| {
//             let mut rising: Option<bool> = None;
//             for i in seq {
//                 if i == 0 || i.abs() > 3 {
//                     return 0;
//                 } else if rising.is_none() {
//                     rising = Some(i > 0);
//                 } else if rising.unwrap() != (i > 0) {
//                     return 0;
//                 }
//             }
//             return 1;
//         })
//         .inspect(|w| println!("{:?}", w))
//         .reduce(|a,b| a+b);
//     println!("{:?}", res);
// }
