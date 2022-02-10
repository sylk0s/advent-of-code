use std::fs; 

fn main() {
    let file = fs::read_to_string("input8.txt")
        .expect("Could not read file");

    let lines: Vec<&str> = file.split("\n").collect();

    println!("{}",part_one(&lines));
    // println!("{}",part_two(&lines));
}

fn part_one(lines: &Vec<&str>) -> i32 {
    
    // transforms lines into each seperate code
    let mut end_line: Vec<&str> = Vec::new();
    for line in lines {
        let second_half: Vec<&str> = line.split("|").collect();
        let codes = second_half[1];
        let codes_vec: Vec<&str> = codes.split(" ").collect();
        for i in 0..codes_vec.len() {
            end_line.push(codes_vec[i].clone()); 
        }
    }

    // checks the codes for specific lengths
    let mut count = 0;
    for code in end_line {
        println!("{} - len of {}",code,code.chars().count());
        if [2,3,4,7].contains(&code.chars().count()) {
            count += 1;
        } 
    }
    count
}