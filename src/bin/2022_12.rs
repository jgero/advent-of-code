use std::{fs, fmt::Display};

fn main() {
    let file_content = fs::read_to_string("./rsc/hill_layout.txt").expect("should have been able to read the file");
    let mut map = HeightMap::new(&file_content);
    println!("shortest route from start to end is {} steps", map.get_shortest_route(None, None).unwrap());
}

#[derive(Copy)]
struct Coordiante {
    x: usize,
    y: usize
}

impl Coordiante {
    pub fn get_adjacent(&self, width: usize, height: usize) -> Vec<Coordiante> {
        let mut ad = Vec::new();
        // left
        if self.x > 0 {
            ad.push(Coordiante{x: self.x - 1, y: self.y});
        }
        // up
        if self.y > 0 {
            ad.push(Coordiante{x: self.x, y: self.y - 1});
        }
        // right
        if self.x + 1 < width {
            ad.push(Coordiante{x: self.x + 1, y: self.y});
        }
        // down
        if self.y + 1 < height {
            ad.push(Coordiante{x: self.x, y: self.y + 1});
        }
        return ad;
    }
}

impl Clone for Coordiante {
    fn clone(&self) -> Self {
        Coordiante { x: self.x, y: self.y }
    }
}

impl Display for Coordiante {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(x: {} y:{})", self.x, self.y)
    }
}

impl PartialEq for Coordiante {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

struct HeightMap {
    // outer vector is y direction and inner one is x direction
    map: Vec<Vec<usize>>,
    start: Coordiante,
    end: Coordiante,
    width: usize,
    height: usize
}

impl HeightMap {
    pub fn new(input: &str) -> Self {
        let mut map = Vec::new();
        let mut width = 0;
        let mut start = Coordiante { x: 0, y: 0 };
        let mut end = Coordiante { x: 0, y: 0 };
        for (y, line ) in input.lines().enumerate() {
            let mut row = Vec::new();
            for (x, c ) in line.chars().enumerate() {
                match c {
                    'S' => {
                        row.push(0);
                        start = Coordiante{ x, y };
                    },
                    'E' => {
                        row.push(26);
                        end = Coordiante{ x, y };
                    },
                    _ => row.push(c as usize - 'a' as usize)
                }
            }
            if width == 0 {
                width = row.len();
            }
            map.push(row);
        }
        let height = map.len();
        Self { map, start, end, width, height }
    }

    pub fn get_shortest_route(&mut self, p: Option<Coordiante>, visited: Option<Vec<usize>>) -> Option<usize> {
        // take provided coordinate or start as current point
        let p = if let Some(val) = p { val } else { self.start };
        // take provided visited map or use new one
        let mut visited = if let Some(val) = visited { val } else { vec![0; self.height] };
        // mark current point as visited
        visited[p.y] |= 1 << p.x;
        // break when end is reached
        if self.end == p {
            return Some(0);
        }
        let mut paths: Vec<usize> = Vec::new();
        // get all addjacent Coordinates within the bounds of the map
        for adjacent in p.get_adjacent(self.width, self.height) {
            // check which of those can be visited
            if self.is_allowed_step(&p, &adjacent, &visited) {
                // DFS: search subtree before checking siblings
                if let Some(val) = self.get_shortest_route(Some(adjacent), Some(visited.clone())) {
                    // add lengths of sub paths to vector
                    paths.push(val + 1);
                }
            }
        }
        if paths.len() == 0 {
            None
        } else {
            paths.sort();
            Some(paths[0])
        }
    }

    fn is_allowed_step(&self, curr: &Coordiante, next: &Coordiante, visited: &Vec<usize>) -> bool {
        // check if elevation diff is too high
        let curr_e = self.map.get(curr.y).unwrap().get(curr.x).unwrap();
        let next_e = self.map.get(next.y).unwrap().get(next.x).unwrap();
        if curr_e.abs_diff(*next_e) > 1 {
            return false;
        }
        // check if the coordiante has been visited already
        if visited.get(next.y).unwrap() >> next.x & 1 == 1 {
            return false;
        }
        return true;
    }
}

