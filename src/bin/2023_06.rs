use std::fs;

fn main() {
    let input = fs::read_to_string("rsc/2023_06.txt").unwrap();
    let lines: Vec<&str> = input.lines().collect();
    let durations: Vec<usize> = lines.get(0).unwrap()
        .strip_prefix("Time:").unwrap()
        .trim()
        .split(" ")
        .filter(|it| !it.is_empty())
        .map(|it| it.parse().unwrap())
        .collect();
    let distances: Vec<usize> = lines.get(1).unwrap()
        .strip_prefix("Distance:").unwrap()
        .trim()
        .split(" ")
        .filter(|it| !it.is_empty())
        .map(|it| it.parse().unwrap())
        .collect();
    let mut games: Vec<Game> = Vec::new();
    for (idx, dur) in durations.iter().enumerate() {
        games.push(Game { duration: *dur, distance_to_beat: *distances.get(idx).unwrap() });
    }
    let res: usize = games.iter()
        .map(|game| game.get_winning_button_durations().len())
        .reduce(|a,b| a * b).unwrap();
    println!("multiplied options: {}", res);
    let long_duration = durations.iter().map(|it| it.to_string()).collect::<String>().parse::<usize>().unwrap();
    let long_distance = distances.iter().map(|it| it.to_string()).collect::<String>().parse::<usize>().unwrap();
    let long_game = Game{ duration: long_duration, distance_to_beat: long_distance };
    println!("there are {} possiblities for the long game", long_game.get_winning_button_durations().len());
}

#[derive(Debug)]
struct Game {
    duration: usize,
    distance_to_beat: usize
}

impl Game {
    fn get_winning_button_durations(&self) -> Vec<usize> {
        let mut winning = Vec::<usize>::new();
        for press_duration in 0.. {
            if press_duration >= self.duration {
                break;
            }
            if self.is_winning_button_duration(press_duration) {
                winning.push(press_duration);
            } else if !winning.is_empty() {
                // winning curve is over, stopping here is safe
                break;
            }
        }
        winning
    }

    // f(x) = x (a * x) - b
    //
    // x: button press duration
    // a: game duration
    // b: distance to beat
    //
    // for all x where y > 0 the resulting distance beats the record
    fn is_winning_button_duration(&self, press_duration: usize) -> bool {
        press_duration as isize * (self.duration as isize - press_duration as isize) - self.distance_to_beat as isize > 0
    }
}
