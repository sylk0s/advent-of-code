use std::fs; 

fn main() {
    let file = fs::read_to_string("input8.txt")
        .expect("Could not read file");

    let lines: Vec<&str> = file.split("\n").collect();

    println!("{}",part_one(&lines));
    // println!("{}",part_two(&lines));
}

fn part_one(lines: &Vec<&str>) -> i32 {
    let mut end_line: Vec<Vec<&str>> = Vec::new();
    for line in lines {
        let second_half: Vec<&str> = line.split("|").collect();
        let codes = second_half[1];
        let codes_vec: Vec<&str> = second_half.split(" ").collect();
        end_line.push(codes_vec.clone()); 
    }
    0
}