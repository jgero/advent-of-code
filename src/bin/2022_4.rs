use std::fs;

fn main() {
    let file_content = fs::read_to_string("./rsc/section_assignments.txt").expect("should have been able to read the file");
    for line in file_content.lines() {
        let a = parse_assignments(line);
        if a[0].contains(&a[1]) || a[1].contains(&a[0]) {
            println!("{} has fully overlapping assignments", line);
        }
    }
}

struct Range {
    min: u16,
    max: u16
}

impl Range {
    pub fn contains(&self, other: &Range) -> bool {
        if other.min < self.min {
            return false;
        }
        if other.max > self.max {
            return false;
        }
        return true;

    }
}

// expect a string like "12-12,12-12" and returns a vector of ranges
fn parse_assignments(input: &str) -> Vec<Range> {
    input.split(",").map(|el| -> Range { parse_range(el)}).collect()
}

// expect a string of the format "12-34" and return it as range
fn parse_range(input: &str) -> Range {
    let parts: Vec<u16> = input.split("-").map(|el| -> u16 {
        el.parse::<u16>().unwrap()
    }).collect();
    return Range{
        min: parts[0],
        max: parts[1]
    };
}
