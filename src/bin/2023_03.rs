use std::fs;

fn main() {
    let input = fs::read_to_string("rsc/2023_03.txt").unwrap();
    let b = Blueprint::try_from(input.as_str()).unwrap();
    println!("part number sum: {:?}", b.part_number_sum());
    println!("gear ratio sum: {:?}", b.gear_ratio_sum());
}

impl TryFrom<&str> for Blueprint {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut part_numbers: Vec<Number> = Vec::new();
        let mut parts: Vec<Part> = Vec::new();
        value.lines()
            .into_iter().enumerate()
            .for_each(|( line_idx, line )| {
                let mut iter = line.chars().into_iter().enumerate();
                let mut active_num: Option<Number> = None;
                while let Some(( char_idx, c )) = iter.next() {
                    if c.is_digit(10) {
                        if let Some(num) = active_num.as_mut() {
                            num.buffer.push(c);
                            num.width += 1;
                        } else {
                            active_num = Some(Number { value: 0, buffer: c.to_string(), width: 1, coord: Coord { x: char_idx + 1, y: line_idx + 1 } });
                        }
                        continue;
                    } else {
                        if let Some(mut num) = active_num {
                            num.finalize();
                            part_numbers.push(num);
                            active_num = None;
                        }
                    }
                    if c == '.' {
                        continue;
                    }
                    parts.push(Part { symbol: c, coord: Coord { x: char_idx + 1, y: line_idx + 1 }});
                }
                if let Some(mut num) = active_num {
                    num.finalize();
                    part_numbers.push(num);
                }
            });
        Ok(Blueprint { parts, part_numbers })
    }
}

#[derive(Debug)]
struct Blueprint {
    parts: Vec<Part>,
    part_numbers: Vec<Number>
}

impl Blueprint {
    fn part_number_sum(&self) -> usize {
        self.part_numbers.iter()
            .filter(|num| {
                let min_x = num.coord.x - 1;
                let max_x = num.coord.x + num.width;
                let min_y = num.coord.y - 1;
                let max_y = num.coord.y + 1;
                let mut iter = self.parts.iter();
                while let Some(p) = iter.next() {
                    if ( min_x..=max_x ).contains(&p.coord.x) && (min_y..=max_y).contains(&p.coord.y) {
                        return true;
                    }
                }
                return false;
            })
        .map(|num| num.value)
            .reduce(|a,b| a + b).unwrap().try_into().unwrap()
    }

    fn gear_ratio_sum(&self) -> usize {
        self.parts.iter()
            .filter(|p| p.symbol == '*')
            .filter_map(|part| {
                let min_x = part.coord.x - 1;
                let max_x = part.coord.x + 1;
                let min_y = part.coord.y - 1;
                let max_y = part.coord.y + 1;
                let adjacent: Vec<Number> = self.part_numbers.iter()
                    .cloned()
                    .filter(|num| {
                        (min_x <= ( num.coord.x + num.width -1 ) && num.coord.x <= max_x
                        || min_x >= ( num.coord.x + num.width -1 ) && num.coord.x >= max_x)
                        && (min_y..=max_y).contains(&num.coord.y)
                    })
                    .collect();
                if adjacent.len() != 2 {
                    return None;
                } else {
                    Some(adjacent.get(0).unwrap().value * adjacent.get(1).unwrap().value)
                }
                    
            })
        .reduce(|a,b| a+b).unwrap()
    }
}

#[derive(Debug,Clone)]
struct Number {
    value: usize,
    buffer: String,
    width: usize,
    coord: Coord
}

impl Number {
    fn finalize(&mut self) {
        self.value = self.buffer.parse().unwrap();
    }
}

#[derive(Debug,Clone)]
struct Part {
    symbol: char,
    coord: Coord
}

#[derive(Debug,Clone)]
struct Coord {
    x: usize,
    y: usize
}
