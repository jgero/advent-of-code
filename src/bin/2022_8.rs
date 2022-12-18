use std::fs;

fn main() {
    let file_content = fs::read_to_string("./rsc/tree_map.txt").expect("should have been able to read the file");

    let mut f = Forest::new(file_content);
    f.calculate_visible_trees();
    print!("forest has {} visible trees", f.get_visible_tree_amount());
}

struct Forest {
    trees: Vec<Vec<usize>>,
    visible_bit: Vec<i32>
}

impl Forest {
    pub fn new(input: String) -> Forest {
        let mut rows = Vec::new();
        let mut visible_bit = Vec::new();
        for line in input.lines() {
            let mut row = Vec::new();
            for char in line.chars() {
                row.push(char.to_digit(10).unwrap().try_into().unwrap())
            }
            rows.push(row);
            visible_bit.push(0);
        }
        return Forest{trees: rows, visible_bit};
    }

    pub fn calculate_visible_trees(&mut self) {
        let height = self.trees.len();
        let width = self.trees.get(0).unwrap().len();

        for (i, row) in self.visible_bit.iter_mut().enumerate() {
            let row_val = row;
            // first and last row are completely visible
            if i == 0 || i == height - 1 {
                for j in 0..width {
                    *row_val |= 1 << j;
                }
                continue;
            }
            // first and last tree in all other rows are also always visible
            *row_val |= 1;
            *row_val |= 1 << width - 1;
            // all other values have to be checked
            for j in 1..width - 1 {
                let mut visible = false;
                
                if !visible {
                    let mut from_top = true;
                    for k in 0..i {
                        if self.trees.get(k).unwrap().get(j).unwrap() >= self.trees.get(i).unwrap().get(j).unwrap() {
                            from_top = false;
                        }

                    }
                    visible = visible || from_top;
                }

                if !visible {
                    let mut from_bottom = true;
                    for k in i+1..width {
                        if self.trees.get(k).unwrap().get(j).unwrap() >= self.trees.get(i).unwrap().get(j).unwrap() {
                            from_bottom = false;
                        }

                    }
                    visible = visible || from_bottom;
                }

                if !visible {
                    let mut from_left = true;
                    for k in 0..j {
                        if self.trees.get(i).unwrap().get(k).unwrap() >= self.trees.get(i).unwrap().get(j).unwrap() {
                            from_left = false;
                        }

                    }
                    visible = visible || from_left;
                }

                if !visible {
                    let mut from_right = true;
                    for k in j+1..width {
                        if self.trees.get(i).unwrap().get(k).unwrap() >= self.trees.get(i).unwrap().get(j).unwrap() {
                            from_right = false;
                        }

                    }
                    visible = visible || from_right;
                }

                if visible {
                    *row_val |= 1 << j;
                }
            }
        }
    }

    pub fn get_visible_tree_amount(&self) -> usize {
        let width = self.trees.get(0).unwrap().len();
        let mut acc = 0;
        for vis_bit in self.visible_bit.iter() {
            for j in 0..width {
                // check if bit at position j is a 1
                if vis_bit >> j & 1 == 1 {
                    acc += 1;
                }
            }
        }
        return acc;
    }
}
