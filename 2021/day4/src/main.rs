use std::fs;

fn main() {
    let file = fs::read_to_string("input3.txt")
        .expect("Could not read file");

    let lines: Vec<&str> = file.split("\n").collect();

    println!("{}",part_one(&lines));
    //println!("{}",part_two(&lines));
}

fn part_one(lines: &Vec<&str>) {
    let called_nums = lines[0];

    let mut boards = Vec::new();

    // creates boards 
    let mut row = 0;
    let mut board = Board::new();
    // theres definitly a better way to do this but im lazy
    for n in 2..called_nums.len() {
        if row > 5 {
            boards.push(board);
            board = Board::new();
            row = 0;
        } else {
            row += 1;
            line = [
                lines[n][0..2].parse::<u32>().expect("Parsing error"), 
                lines[n][3..5].parse::<u32>().expect("Parsing error"), 
                lines[n][6..8].parse::<u32>().expect("Parsing error"), 
                lines[n][9..11].parse::<u32>().expect("Parsing error"), 
                lines[n][12..14].parse::<u32>().expect("Parsing error")];
            if row == 1 { board.l1 = line }
            if row == 2 { board.l2 = line }
            if row == 3 { board.l3 = line }
            if row == 4 { board.l4 = line }
            if row == 5 { board.l5 = line }
        }
    }
    boards.push(board);


}

struct Board {
    l1: [u32; 5],
    l2: [u32; 5],
    l3: [u32; 5],
    l4: [u32; 5],
    l5: [u32; 5],
}

