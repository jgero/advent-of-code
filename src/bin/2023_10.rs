use std::sync::{RwLock, Arc};

const DATA: &str = include_str!("../../rsc/2023_10.txt");

fn main() {
    let mut map = Map::try_from(DATA).unwrap();
    // println!("{:?}", map.get_startpoints());
    for coord in map.get_startpoints() {
        println!("{:?}", map.walk(None, coord, 0));
        
    }
    println!("{:?}", map.get_start_corner());
    println!("{:?}", map.get_max_distance());
    println!("{:?}", map.get_enclosed_tiles());
}

type Coord = (isize, isize);

fn is_in_bounds(c: &Coord) -> bool {
    c.0 >= 0 && c.1 >= 0
}

#[derive(Debug)]
struct Map {
    corners: Vec<Vec<Option<Corner>>>, 
    start: Coord
}

impl Map {
    fn get_enclosed_tiles(&self) -> usize {
        let mut enclosed: usize = 0;
        let mut inside: bool;
        let mut north: bool; 
        let mut south: bool;
        for (line_idx, line) in self.corners.iter().enumerate() {
            inside = false;
            north = false;
            south = false;
            for (tile_idx, tile) in line.iter().enumerate() {
                let maybe_cornwer: Option<Corner> = tile.clone().or_else(|| {
                    if (line_idx as isize, tile_idx as isize) == self.start {
                        Some(self.get_start_corner().clone())
                    } else {
                        None
                    }
                });
                if let Some(corner) = maybe_cornwer {
                    if *corner.distance.read().unwrap() < usize::MAX {
                        match corner.t {
                            CornerType::WN | CornerType::NO => {
                                north = !north;
                            },
                            CornerType::OS | CornerType::SW => {
                                south = !south;
                            },
                            CornerType::NS => {
                                north = !north;
                                south = !south;
                            }
                            _ => {},
                        }
                    if north && south {
                        inside = !inside;
                        north = false;
                        south = false;
                    }
                    print!(".");
                        continue;
                    } else {
                        north = false;
                        south = false;
                    }
                }
                if inside  {
                    print!("X");
                    enclosed += 1;
                } else {
                    print!(".");
                }
            }
            print!("\n");
        }
        enclosed
    }

    fn walk(&mut self, prev: Option<Coord>, visit: Coord, distance: usize) {
        let prev = prev.unwrap_or(self.start);
        if let Some(ref next_corner) = self.corners.get(visit.0 as usize).unwrap().get(visit.1 as usize).unwrap() {
            if distance + 1 < *next_corner.distance.read().unwrap() {
                // write new distance if it's smaller
                *next_corner.distance.write().unwrap() = distance + 1;
                // if there is a next corner
                if let Some(next) = next_corner.next(&prev, &visit) {
                    self.walk(Some(visit), next, distance + 1)
                }
            }
        }
    }

    fn get_max_distance(&self) -> usize {
        self.corners.iter()
            .flatten()
            .filter_map(|maybe_c| maybe_c.clone().map(|c| c.distance))
            .map(|c| *c.read().unwrap())
            .filter(|distance| distance < &usize::MAX)
            .max().unwrap()
    }

    fn get_start_corner(&self) -> Corner {
        let st = self.get_startpoints();
        if st.iter().all(|p| p.0 == self.start.0) {
            Corner {distance: Arc::new( 0.into() ), t: CornerType::OW}
        } else if st.iter().all(|p| p.1 == self.start.1) {
            Corner {distance: Arc::new( 0.into() ), t: CornerType::NS}
        } else if st.iter().find(|p| p.0 == self.start.0 + 1 && p.1 == self.start.1).is_some() {
            // has one point in south
            if st.iter().find(|p| p.0 == self.start.0 && p.1 == self.start.1 + 1).is_some() {
                Corner {distance: Arc::new( 0.into() ), t: CornerType::OS}
            } else {
                Corner {distance: Arc::new( 0.into() ), t: CornerType::SW}
            }
        } else if st.iter().find(|p| p.0 == self.start.0 - 1 && p.1 == self.start.1).is_some() {
            // has one point in north
            if st.iter().find(|p| p.0 == self.start.0 && p.1 == self.start.1 + 1).is_some() {
                Corner {distance: Arc::new( 0.into() ), t: CornerType::NO}
            } else {
                Corner {distance: Arc::new( 0.into() ), t: CornerType::WN}
            }
        } else {
            unreachable!()
        }
    }

    fn get_startpoints(&self) -> Vec<Coord> {
        let mut startpoints: Vec<Coord> = Vec::new();
        // left
        if self.start.1 > 0 {
            if let Some(n) = self.corners.get(self.start.0 as usize).unwrap().get(( self.start.1 - 1 ) as usize).unwrap() {
                // println!("{:?}", n);
                match n.t {
                    CornerType::NO | CornerType::OS | CornerType::OW => startpoints.push((self.start.0, self.start.1 - 1)),
                    _ => {}
                }
            }
        }
        // up
        if self.start.0 > 0 {
            if let Some(n) = self.corners.get(( self.start.0 - 1 ) as usize).unwrap().get(self.start.1 as usize).unwrap()  {
                // println!("{:?}", n);
                match n.t {
                    CornerType::SW | CornerType::OS | CornerType::NS => startpoints.push((self.start.0 - 1, self.start.1)),
                    _ => {}
                }
            }
        }
        // right
        if let Some(n) = self.corners.get(self.start.0 as usize).unwrap().get(( self.start.1 + 1 ) as usize).unwrap() {
            // println!("{:?}", n);
            match n.t {
                CornerType::SW | CornerType::WN | CornerType::OW => startpoints.push((self.start.0, self.start.1 + 1)),
                _ => {}
            }
        }
        // down
        if let Some(n) = self.corners.get(( self.start.0 + 1 ) as usize).unwrap().get(self.start.1 as usize).unwrap() {
            // println!("{:?}", n);
            match n.t {
                CornerType::NO | CornerType::NS | CornerType::WN => startpoints.push((self.start.0 + 1, self.start.1)),
                _ => {}
            }
        }
        startpoints.iter().filter(|s| is_in_bounds(&s)).map(|s| s.to_owned()).collect()
    }
}

impl TryFrom<&str> for Map {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut start: Coord = (0,0);
        let corners: Vec<Vec<Option<Corner>>> = value.lines()
            .enumerate()
            .map(|(line_idx, line)| {
                line.chars()
                    .enumerate()
                    .map(|(char_idx, c)| {
                        if c == 'S' {
                            start = (line_idx as isize, char_idx as isize);
                        }
                        Corner::try_from(c).ok()
                    })
                    .collect()
            })
            .collect();
        Ok(Self { corners: corners.into(), start })
    }
}

#[derive(Debug, Clone)]
struct Corner {
    t: CornerType,
    distance: Arc<RwLock<usize>>,
}

impl Corner {
    fn next(&self, prev: &Coord, my_coords: &Coord) -> Option<Coord> {
        let next = match self.t {
            CornerType::NO => if prev.0 == my_coords.0 { (prev.0 - 1, prev.1 - 1) } else { (prev.0 + 1, prev.1 + 1) },
            CornerType::OS => if prev.0 == my_coords.0 { (prev.0 + 1, prev.1 - 1) } else { (prev.0 - 1, prev.1 + 1) },
            CornerType::SW => if prev.0 == my_coords.0 { (prev.0 + 1, prev.1 + 1) } else { (prev.0 - 1, prev.1 - 1) },
            CornerType::WN => if prev.0 == my_coords.0 { (prev.0 - 1, prev.1 + 1) } else { (prev.0 + 1, prev.1 - 1) },
            CornerType::NS => if prev.0 < my_coords.0 { (prev.0 + 2, prev.1) } else { (prev.0 - 2, prev.1) },
            CornerType::OW => if prev.1 < my_coords.1 { (prev.0, prev.1 + 2) } else { (prev.0, prev.1 - 2) },
        };
        if is_in_bounds(&next) {
            Some(next)
        } else {
            None
        }
    }
    
}

impl TryFrom<char> for Corner {
    type Error = ();
    fn try_from(value: char) -> Result<Self, Self::Error> {
        if let Ok(t) = CornerType::try_from(value) {
            Ok(Self {t, distance: Arc::new(usize::MAX.into()) })
        } else {
            Err(())
        }
    }
}

#[derive(Debug, Clone)]
enum CornerType {
    NO,
    OS,
    SW,
    WN,
    NS,
    OW
}

impl TryFrom<char> for CornerType {
    type Error = ();
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'L' => Ok(Self::NO),
            'F' => Ok(Self::OS),
            '7' => Ok(Self::SW),
            'J' => Ok(Self::WN),
            '|' => Ok(Self::NS),
            '-' => Ok(Self::OW),
            _ => Err(())
        }
    }
}
