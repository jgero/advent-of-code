use std::{fs, str::Chars, borrow::BorrowMut};

fn main() {
    let file_content = fs::read_to_string("./rsc/distress_signal_packets.txt").expect("should have been able to read the file");
    let mut acc = 0;
    for (i, pair ) in file_content.split("\n\n").enumerate() {
        let mut p = PacketPair::new(pair);

        if p.has_correct_order() {
            let index = i + 1;
            println!("index {} is ordered correctly", index);
            acc += index;
        }
    }
    println!("sum of correctly ordered indices is {}", acc);
}

struct PacketPair {
    left: DataItemContainer,
    right: DataItemContainer
}

impl PacketPair {
    pub fn new(input: &str) -> Self {
        let mut p_it = input.split("\n");
        // first line is left
        let mut it = p_it.next().unwrap().chars();
        // skip first "["
        it.next();
        let left = DataItemContainer::parse(it.borrow_mut());
        // second line is right
        let mut it = p_it.next().unwrap().chars();
        // skip first "["
        it.next();
        let right = DataItemContainer::parse(it.borrow_mut());
        // let mut left = ListItem::new();
        // let mut right = ListItem::new();
        PacketPair { left, right }
    }

    pub fn has_correct_order(&mut self) -> bool {
        match self.left.is_ordered(&mut self.right) {
            CompResult::EQ | CompResult::LT => true,
            CompResult::GT => false
        }
    }
}

#[derive(PartialEq)]
enum DataItemType {
    Integer,
    List
}

struct DataItemContainer {
    t: DataItemType,
    value: usize,
    items: Vec<DataItemContainer>
}

impl DataItemContainer {
    pub fn new_int_from_char(next: char) -> DataItemContainer {
        DataItemContainer {
            t: DataItemType::Integer,
            value: next.to_digit(10).unwrap().try_into().unwrap(),
            items: Vec::new()
        }
    }
    pub fn new_int(value: usize) -> DataItemContainer {
        DataItemContainer {t: DataItemType::Integer, value, items: Vec::new()}
    }

    pub fn parse(it: &mut Chars) -> Self {
        let mut items = Vec::new();
        loop {
            let next = it.next().unwrap();
            if next == ']' { break; }
            else if next.is_numeric() {
                items.push(DataItemContainer::new_int_from_char(next));
            } else if next == '[' {
                items.push(DataItemContainer::parse(it))
            } else if next == ',' {
                // do nothing when reading comma
            } else {
                panic!("unexpected value {} when parsing list item", next);
            }
        }
        DataItemContainer { items, value: 0, t: DataItemType::List }
    }

    pub fn to_list(&mut self) {
        if self.t == DataItemType::List {
            return;
        }
        self.t = DataItemType::List;
        self.items = vec![DataItemContainer::new_int(self.value)];
        self.value = 0;
    }

    // returning boolean does not suffice, I need to know if it's equal, greater or less since the
    // comparison only continues until the first lower value for the left side was found
    pub fn is_ordered(&mut self, right: &mut DataItemContainer) -> CompResult {
        if self.t != right.t {
            // one has to be converted to list then
            self.to_list();
            right.to_list();
        }
        match self.t {
            DataItemType::Integer => {
                if self.value == right.value {
                    CompResult::EQ
                } else if self.value < right.value {
                    CompResult::LT
                } else {
                    CompResult::GT
                }
            },
            DataItemType::List => {
                for i in 0..self.items.len().max(right.items.len()) {
                    let i_l = self.items.get_mut(i);
                    let i_r = right.items.get_mut(i);
                    if i_l.is_none() && i_r.is_none() {
                        // both are empty so return true
                        panic!("this case is never reached because loop breaks");
                        // return CompResult::EQ;
                    } else if i_l.is_none() && i_r.is_some() {
                        // left is empty first
                        return CompResult::LT;
                    } else if i_l.is_some() && i_r.is_none() {
                        // right is empty first
                        return CompResult::GT;
                    } else {
                        // if both sides have items return early if they are not equal
                        let i_l = i_l.unwrap();
                        let i_r = i_r.unwrap();
                        match i_l.is_ordered(i_r) {
                            CompResult::EQ => {},
                            CompResult::GT => return CompResult::GT,
                            CompResult::LT => return CompResult::LT
                        }
                    }
                }
                // left and right have mathing amount of elements and are all equal
                return CompResult::EQ;
            }
        }
    }
}

enum CompResult {
    EQ,
    GT,
    LT
}
