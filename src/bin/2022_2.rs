use std::{fs, char};

#[derive(PartialEq, Eq)]
enum HandShape {
    ROCK,
    PAPER,
    SCISSORS
}

impl HandShape {
    pub fn from_char(c: char) -> HandShape {
        match c {
            'A' | 'X' => HandShape::ROCK,
            'B' | 'Y' => HandShape::PAPER,
            'C' | 'Z' => HandShape::SCISSORS,
            unknown => panic!("cannot infer hand shape from char {}", unknown)
        }
    }

    pub fn score(&self) -> usize {
        match self {
            HandShape::ROCK => 1,
            HandShape::PAPER => 2,
            HandShape::SCISSORS => 3
        }
    }
}

fn main() {
    let file_content = fs::read_to_string("./rsc/strategy_guide.txt").expect("should have been able to read the file");
    let mut score = 0;
    for l in file_content.lines() {
        let mut char_it = l.chars();
        let theirs = HandShape::from_char(char_it.next().unwrap());
        if char_it.next().unwrap() != ' ' {
            panic!("expected a whitespace here");
        }
        let mine = HandShape::from_char(char_it.next().unwrap());
        score += mine.score() + get_my_score(theirs, mine);
    }
    println!("my final score is {}", score);
}

fn get_my_score(theirs: HandShape, mine: HandShape) -> usize {
    if theirs == mine {
        return 3;
    }
    match theirs {
        HandShape::ROCK => {
            match mine {
                HandShape::PAPER => 6,
                HandShape::SCISSORS => 0,
                _ => panic!("this is impossible")
            }
        },
        HandShape::PAPER => {
            match mine {
                HandShape::SCISSORS => 6,
                HandShape::ROCK => 0,
                _ => panic!("this is impossible")
            }
        },
        HandShape::SCISSORS => {
            match mine {
                HandShape::ROCK => 6,
                HandShape::PAPER => 0,
                _ => panic!("this is impossible")
            }
        }
        
    }
}
