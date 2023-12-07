use std::{fs, collections::HashMap};

fn main() {
    let input = fs::read_to_string("rsc/2023_07.txt").unwrap();
    let mut hands: Vec<(Hand, usize)> = input.lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
        let parts: Vec<&str> = line.split(" ").collect();
        (Hand::try_from(*parts.get(0).unwrap()).unwrap(), parts.get(1).unwrap().parse().unwrap())
    }).collect();
    hands.sort_by_key(|it| it.0.clone());
    let winnings = hands.iter().enumerate()
        .map(|(idx, (_, bid))| (idx + 1) * bid)
        .reduce(|a,b| a+b).unwrap();
    println!("winnings: {}", winnings);
}

#[derive(PartialEq, Eq, Debug, Clone)]
enum Hand {
    FiveOfKind(String),
    FourOfKind(String),
    FullHouse(String),
    ThreeOfKind(String),
    TwoPair(String),
    Pair(String),
    HighCard(String),
}

impl Hand {
    fn get_discriminator(&self) -> u8 {
        match self {
            Self::FiveOfKind(_) => 6,
            Self::FourOfKind(_) => 5,
            Self::FullHouse(_) => 4,
            Self::ThreeOfKind(_) => 3,
            Self::TwoPair(_) => 2,
            Self::Pair(_) => 1,
            Self::HighCard(_) => 0,
        }
    }

    fn is_stronger_first_cards(&self, other: &Self) -> Option<bool> {
        let mut self_hand_it = match self {
            Self::FiveOfKind(hand) => hand,
            Self::FourOfKind(hand) => hand,
            Self::FullHouse(hand) => hand,
            Self::ThreeOfKind(hand) => hand,
            Self::TwoPair(hand) => hand,
            Self::Pair(hand) => hand,
            Self::HighCard(hand) => hand,
        }.chars();
        let mut other_hand_it = match other {
            Self::FiveOfKind(hand) => hand,
            Self::FourOfKind(hand) => hand,
            Self::FullHouse(hand) => hand,
            Self::ThreeOfKind(hand) => hand,
            Self::TwoPair(hand) => hand,
            Self::Pair(hand) => hand,
            Self::HighCard(hand) => hand,
        }.chars();
        while let (Some(self_card), Some(other_card)) = ( self_hand_it.next(), other_hand_it.next() ) {
            if char_to_discriminator(&self_card).unwrap() != char_to_discriminator(&other_card).unwrap() {
                return Some(char_to_discriminator(&self_card).unwrap() > char_to_discriminator(&other_card).unwrap());
            }
        }
        return None;
    }
}

fn char_to_discriminator(c: &char) -> Option<u8> {
    match c {
        'J' => Some(0),
        '2' => Some(1),
        '3' => Some(2),
        '4' => Some(3),
        '5' => Some(4),
        '6' => Some(5),
        '7' => Some(6),
        '8' => Some(7),
        '9' => Some(8),
        'T' => Some(9),
        'Q' => Some(10),
        'K' => Some(11),
        'A' => Some(12),
        _ => None
    }
}

impl TryFrom<&str> for Hand {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
         let mut stacks = HashMap::<char, u8>::new();
         let mut jokers = 0;
         value.chars().into_iter().for_each(|c| {
             if c == 'J' {
                 jokers += 1;
             } else {
                 stacks.insert(c, stacks.get(&c).unwrap_or(&0) + 1);
             }
         });
         let hand = value.to_string();
         match ( stacks.len(), *stacks.values().max().unwrap_or(&0) + jokers) {
             (_, 5) => Ok(Self::FiveOfKind(hand)),
             (_, 4) => Ok(Self::FourOfKind(hand)),
             (2, 3) => Ok(Self::FullHouse(hand)),
             (3, 3) => Ok(Self::ThreeOfKind(hand)),
             (3, 2) => Ok(Self::TwoPair(hand)),
             (4, 2) => Ok(Self::Pair(hand)),
             (_, 1) => Ok(Self::HighCard(hand)),
             _ => Err(())
         }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.get_discriminator() == other.get_discriminator() {
            if let Some(stronger) = self.is_stronger_first_cards(other) {
                if stronger {
                    Some(std::cmp::Ordering::Greater)
                } else {
                    Some(std::cmp::Ordering::Less)
                }
            } else {
                Some(std::cmp::Ordering::Equal)
            }
        } else {
            if self.get_discriminator() < other.get_discriminator() {
                    Some(std::cmp::Ordering::Less)
            } else {
                    Some(std::cmp::Ordering::Greater)
            }
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.get_discriminator() == other.get_discriminator() {
            if let Some(stronger) = self.is_stronger_first_cards(other) {
                if stronger {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Less
                }
            } else {
                std::cmp::Ordering::Equal
            }
        } else {
            if self.get_discriminator() < other.get_discriminator() {
                    std::cmp::Ordering::Less
            } else {
                    std::cmp::Ordering::Greater
            }
        }
    }
}
