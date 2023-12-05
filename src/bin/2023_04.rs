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
    original_cards: Vec<Card>
}

impl CardStack {
    fn new(input: &str) -> CardStack {
        let original_cards = input.lines()
            .map(|it| Card::try_from(it).unwrap())
            .collect();
        CardStack { original_cards }
    }

    fn play_stack_game(&mut self) -> usize {
        let mut stack: HashMap<usize, usize> = HashMap::new();
        self.original_cards.iter().for_each(|c| {
             stack.insert(c.id, 1);
        });
        self.original_cards.iter().for_each(|c| {
            let upper_bound_copies = c.get_winnig_number_amount();
            if upper_bound_copies == 0 {
                return;
            }
            for copy_index in (c.id + 1)..=(c.id + upper_bound_copies) {
                let stack_val = stack.get(&c.id).unwrap().clone();
                // println!("curr {}", curr);
                stack.insert(copy_index, stack_val + stack.get(&copy_index).or(Some(&1)).unwrap());
            }
        });
        return stack.values().map(|it| *it).reduce(|a,b| a+b).unwrap();
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
