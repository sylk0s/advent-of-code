use std::fs;

fn main() {
    let file = fs::read_to_string("input4.txt")
        .expect("Could not read file");

    let lines: Vec<&str> = file.split("\n").collect();

    println!("{}",part_one(&lines));
    //println!("{}",part_two(&lines));
}

fn part_one(lines: &Vec<&str>) -> u32 {
    // get numbers called
    let called_nums_str: Vec<&str> = lines[0].split(',').collect();

    let mut called_nums = Vec::new();
    for n in called_nums_str {
        called_nums.push(n.trim().parse::<u32>().expect("Parsing error"));
    }

    let boards = generate_boards(lines);

    for num in called_nums {
        for board in &boards {
            board.mark_num(num);
            if board.has_won() {
                return board.calculate_total();
            }
        }
    }
    0
}

// this method is NOT good. I could do this so much faster in python or something where im familiar with the lists and such
fn generate_boards(lines: &Vec<&str>) -> Vec<Board> {
    let mut boards = Vec::new();

    // creates boards 
    let mut row = 0;
    let mut board = Board {l1:[0;5],l2:[0;5],l3:[0;5],l4:[0;5],l5:[0;5],};
    

    // theres definitly a better way to do this but im lazy
    for n in 2..lines.len() {
        if row > 4 {
            boards.push(board);
            board = Board {l1:[0;5],l2:[0;5],l3:[0;5],l4:[0;5],l5:[0;5],};
            row = 0;
        } else {
            row += 1;
            let line = [
                lines[n][0..2].trim().parse::<u32>().expect("Parsing error"), 
                lines[n][3..5].trim().parse::<u32>().expect("Parsing error"), 
                lines[n][6..8].trim().parse::<u32>().expect("Parsing error"), 
                lines[n][9..11].trim().parse::<u32>().expect("Parsing error"), 
                lines[n][12..14].trim().parse::<u32>().expect("Parsing error")];
            if row == 1 { board.l1 = line }
            if row == 2 { board.l2 = line }
            if row == 3 { board.l3 = line }
            if row == 4 { board.l4 = line }
            if row == 5 { board.l5 = line }
        }
    }
    boards.push(board);

    boards
}

// maybe not the best way to do this but I wanted to use something different
struct Board {
    l1: [u32; 5],
    l2: [u32; 5],
    l3: [u32; 5],
    l4: [u32; 5],
    l5: [u32; 5],
}

impl Board {
    fn has_won(&self) -> bool {
        let rows = [self.l1, self.l2, self.l3, self.l4, self.l5];
        for row in rows {
            if row == [0;5] {
                return true;
            }
        }
        let mut count = 0;
        for n in 0..5 {
            for row in rows {
                if row[n] == 0 {
                    count += 1;
                }
            }
        }
        if count == 5 {
            return true;
        }
        false
    }

    fn calculate_total(&self) -> u32 {
        let rows = [self.l1, self.l2, self.l3, self.l4, self.l5];
        let mut sum = 0;
        for row in rows {
            for n in 0..5 {
                sum += row[n];
            }
        }
        sum
    }

    fn mark_num(&self, num: u32) {
        let rows = [self.l1, self.l2, self.l3, self.l4, self.l5];
        for mut row in rows {
            for n in 0..5 {
                if row[n] == num {
                    row[n] = 0;
                }
            }
        }
    }
}