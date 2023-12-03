use std::fs;

use substring::Substring;

// part 1 
// fn main() {
//     let input = fs::read_to_string("rsc/2023_01.txt").unwrap();
//     let sum = input.lines().into_iter()
//         .map(|line| {
//            let mut num_vec = Vec::new();
//            line.chars()
//                .filter_map(|c| c.to_digit(10))
//                .for_each(|digit| num_vec.push(digit));
//            format!("{}{}", num_vec.first().unwrap(), num_vec.last().unwrap()).parse::<u32>().unwrap()
//         })
//         .reduce(|a, b| a + b);
//     println!("the calibration value sum is {}", sum.unwrap());
// }
//
fn main() {
    let input = fs::read_to_string("rsc/2023_01.txt").unwrap();
    let sum = input.lines().into_iter()
        .map(|line| {
           let mut num_vec = Vec::new();
           line.chars()
               .into_iter().enumerate()
               .for_each(|( i, c )| {
                   if let Some(digit) = c.to_digit(10) {
                       num_vec.push(digit);
                       return;
                   }
                   for size in 3..=5 {
                       if line.len() >= i+size {
                           if let Ok(digit) = StringDigit::try_from(line.substring(i, i+size)) {
                               num_vec.push(digit.into())
                           }
                       }
                   }
                   
               });
           format!("{}{}", num_vec.first().unwrap(), num_vec.last().unwrap()).parse::<u32>().unwrap()
        })
        .reduce(|a, b| a + b);
    println!("the calibration value sum is {}", sum.unwrap());
}

#[derive(Debug)]
enum StringDigit {
    ONE,
    TWO,
    THREE,
    FOUR,
    FIVE,
    SIX,
    SEVEN,
    EIGHT,
    NINE
}

impl Into<u32> for StringDigit {
    fn into(self) -> u32 {
        match self {
            Self::ONE => 1,
            Self::TWO => 2,
            Self::THREE => 3,
            Self::FOUR => 4,
            Self::FIVE => 5,
            Self::SIX => 6,
            Self::SEVEN => 7,
            Self::EIGHT => 8,
            Self::NINE => 9,
        }
    }
}

impl TryFrom<&str> for StringDigit {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() == 3 {
            match value {
                "one" => Ok( Self::ONE ),
                "two" => Ok(Self::TWO),
                "six" => Ok(Self::SIX),
                _ => Err(())
            }
        } else if value.len() == 4 {
            match value {
                "four" => Ok( Self::FOUR ),
                "five" => Ok(Self::FIVE),
                "nine" => Ok(Self::NINE),
                _ => Err(())
            }
        } else if value.len() == 5 {
            match value {
                "three" => Ok( Self::THREE ),
                "seven" => Ok(Self::SEVEN),
                "eight" => Ok(Self::EIGHT),
                _ => Err(())
            }
        } else {
            Err(())
        }
    }
}
