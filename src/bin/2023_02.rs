use std::fs;

// const RED_CUBES: u32 = 12;
// const GREEN_CUBES: u32 = 13;
// const BLUE_CUBES: u32 = 14;

// fn main() {
//     let input = fs::read_to_string("rsc/2023_02.txt").unwrap();
//     let result = input.lines()
//         .map(|line| Game::try_from(line).unwrap())
//         .filter(|game| 
//             !game.draws.iter().any(|draw| draw.red > RED_CUBES || draw.green > GREEN_CUBES || draw.blue > BLUE_CUBES)
//         )
//     .map(|game| game.id)
//         .reduce(|a,b| a+b).unwrap()
//         ;
//     println!("{}", result);
//
// }

fn main() {
    let input = fs::read_to_string("rsc/2023_02.txt").unwrap();
    let result = input.lines()
        .map(|line| Game::try_from(line).unwrap().power())
        .reduce(|a,b| a+b).unwrap()
        ;
    println!("{}", result);

}

struct Draw {
    red: u32,
    green: u32,
    blue: u32
}

impl TryFrom<&str> for Draw {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut red = 0;
        let mut green = 0;
        let mut blue= 0;
        value.split(",").into_iter()
            .map(|s| s.trim())
            .for_each(|s| {
                if s.ends_with(" red") {
                    red += s.strip_suffix(" red").unwrap().parse::<u32>().unwrap();
                } else if s.ends_with(" green") {
                    green += s.strip_suffix(" green").unwrap().parse::<u32>().unwrap();
                } else if s.ends_with(" blue") {
                    blue += s.strip_suffix(" blue").unwrap().parse::<u32>().unwrap();
                }
            });
        Ok(Draw { red, green, blue })
    }
}

struct Game {
    id: u32,
    draws: Vec<Draw>
}

impl Game {
    fn power(&self) -> u32 {
        let min_tuple = self.draws.iter()
            .map(|draw| (draw.red, draw.green, draw.blue))
            .reduce(|a, b| {
                (a.0.max(b.0), a.1.max(b.1), a.2.max(b.2))
                
             }).unwrap();
        println!("{:?}",min_tuple);
        return min_tuple.0 * min_tuple.1 * min_tuple.2;
    }
}

impl TryFrom<&str> for Game {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut iter = value.split(":").into_iter();
        let id = iter.next().unwrap().strip_prefix("Game ").unwrap().parse::<u32>().unwrap();
        let draws = iter.next().unwrap().split(";").map(|s| Draw::try_from(s).unwrap()).collect();
        Ok(Game {id, draws})
    }
}
