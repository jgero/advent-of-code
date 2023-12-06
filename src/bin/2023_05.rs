use std::{fs, ops::Range};

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref MAPPING_EXP: Regex = Regex::new(r"(?sm)(\w*)-to-(\w*) map:\n(.*)").unwrap();
}

type Unsigned = u64;
type Signed = i128;

fn main() {
    let input = fs::read_to_string("rsc/2023_05.txt").unwrap();
    let mappers: Vec<Mapper> = input.split("\n\n")
        .filter(|block| MAPPING_EXP.is_match(block))
        .map(|block| Mapper::try_from(block).unwrap())
        .collect();
    let mut lowest_seed: Unsigned = Unsigned::MAX;
    let seed_ranges: Vec<Unsigned> = input.chars()
        .take_while(|c| *c != '\n')
        .collect::<String>()
        .strip_prefix("seeds: ").unwrap()
        .split(" ")
        .map(|digit| digit.parse().unwrap())
        .collect();
    let mut it = seed_ranges.iter().enumerate();
    it.next();
    let mut it = it.step_by(2);
    while let Some((idx, val)) = it.next() {
        let start = *seed_ranges.get(idx - 1).unwrap();
        let range = start..(start+*val);
        println!("[{}/{}] range {:?}. size hint: {}", (idx + 1)/ 2, seed_ranges.len() / 2, range, range.size_hint().0);
        for seed in range {
            let mapped = map_seed(&seed, &mappers);
            if mapped < lowest_seed {
                lowest_seed = mapped;
            }
        }
    }
    println!("lowest seed: {}", lowest_seed);
}

fn map_seed(seed: &Unsigned, mappers: &Vec<Mapper>) -> Unsigned {
    let mut inter = *seed;
    'mappers: for mapper in mappers {
        for mapping in mapper.mappings.iter() {
            if !mapping.range.contains(&inter) {
                continue;
            } else {
                inter = ( inter as Signed + mapping.offset ) as Unsigned;
                continue 'mappers;
            }
        }
    }
    return inter;
}

#[derive(Debug)]
struct Mapper {
    mappings: Vec<Mapping>
}

#[derive(Debug)]
struct Mapping {
    range: Range<Unsigned>,
    offset: Signed
}

impl TryFrom<&str> for Mapper {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let captures = MAPPING_EXP.captures(value).unwrap();
        let mappings = captures.get(3).unwrap().as_str()
            .split("\n")
            .filter(|it| !it.is_empty())
            .map(|it| Mapping::try_from(it).unwrap())
            .collect();
        Ok(Mapper { mappings })
    }
}

impl TryFrom<&str> for Mapping {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let digits: Vec<Unsigned> = value.split(" ").map(|digits| digits.parse().unwrap()).collect();
        let to_range = digits.get(0).unwrap();
        let from_range = digits.get(1).unwrap();
        let size = digits.get(2).unwrap();
        Ok(Mapping {range: (*from_range..(*from_range+*size)), offset: *to_range as Signed - *from_range as Signed})
    }
}
