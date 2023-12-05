use std::{fs, collections::{HashSet, HashMap}};


fn main() {
    let input = fs::read_to_string("rsc/2023_04.txt").unwrap();
    let part_1 = input.lines()
        .map(|it| Card::try_from(it).unwrap().get_points())
        .reduce(|a,b| a+b).unwrap();
    println!("the card stack contains {} points", part_1);
    let mut stack = CardStack::new(&input);
    let result = stack.play_stack_game();
    println!("part 2 stack size is {}", result);
}

struct CardStack {
    original_cards: HashMap<usize, Card>,
    stack: Vec<usize>
}

impl CardStack {
    fn new(input: &str) -> CardStack {
        let mut original_cards: HashMap<usize, Card> = HashMap::new();
        let mut stack = Vec::new();
        input.lines()
            .for_each(|it| {
                let c = Card::try_from(it).unwrap();
                stack.push(c.id);
                original_cards.insert(c.id, c);
            });
        CardStack { original_cards, stack }
    }

    // oof, this runs for 33 seconds, what's the problem?
    fn play_stack_game(&mut self) -> usize {
        let mut i = 0;
        while i < self.stack.len() {
            let id = self.stack.get(i).unwrap().clone();
            let next = self.original_cards.get(&id).unwrap().get_winnig_number_amount();
            if next == 0 {
                i += 1;
                continue;
            }
            for it in 1..=next {
                self.stack.push(id + it);
            }
            i += 1;
        }
        return self.stack.len();
    }
}

struct Card {
    id: usize,
    winning: HashSet<usize>,
    having: HashSet<usize>
}

impl Card {
     fn get_winnig_number_amount(&self) -> usize {
         self.winning.iter()
             .filter(|it| self.having.contains(it))
             .count()
     }

     fn get_points(&self) -> u32 {
         let hits = self.get_winnig_number_amount();
         if hits == 0 {
             return 0;
         }
         u32::pow(2, u32::try_from(hits).unwrap() - 1)
     }
}

impl TryFrom<&str> for Card {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let id_split = value.split(":").collect::<Vec<&str>>();
        let id: usize = id_split[0].strip_prefix("Card ").unwrap().trim().parse().unwrap();
        let number_sets: Vec<HashSet<usize>> = id_split[1].split("|")
            .map(|s| s.trim())
            .map(|s| s.split(" ").filter(|s| *s != "").map(|s| s.parse().unwrap()).collect::<HashSet<usize>>())
            .collect::<Vec<HashSet<usize>>>();
        return Ok(Card{ id, winning: number_sets[0].to_owned(), having: number_sets[1].to_owned() });
    }
}
