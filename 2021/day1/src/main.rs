use std::fs;
use std::vec::Vec;

fn main() {
    let file = fs::read_to_string("input1.txt")
        .expect("Could not read file");

    let lines: Vec<&str> = file.split("\n").collect();
    let lines2: Vec<&str> = file.split("\n").collect(); // dont like this solution

    println!("The answer to part 1 is {}", part_one(lines));
    println!("The answer to part 2 is {}", part_two(lines2));
}

fn part_one(lines: Vec<&str>) -> u32 {
    let mut is_first = true;
    let mut prev: u32 = 0;
    let mut count = 0;

    for line in lines {
        if is_first {
            is_first = false;
            prev = line.trim().parse().expect("Parsing error");
            continue;
        }
        let current: u32 = line.trim().parse().expect("Parsing error");
        if current > prev {
            count = count + 1;
        }
        prev = current;
    }
    count
}

fn part_two(lines: Vec<&str>) -> u32 {
    let mut window_count = 0;
    let mut count = 0;

    for n in 0..lines.len() {
        if window_count < 3 {
            window_count = window_count + 1;
        } else {
            //im not entirely understanding why this works, shouldnt this be able to simplify to t1>t4?
            let t1: u32 = lines[n].trim().parse::<u32>().expect("Parsing error");
            let t2: u32 = lines[n-1].trim().parse::<u32>().expect("Parsing error");
            let t3: u32 = lines[n-2].trim().parse::<u32>().expect("Parsing error");
            let t4: u32 = lines[n-3].trim().parse::<u32>().expect("Parsing error");
            if t1+t2+t3 > t2+t3+t4  {
                count = count + 1; // is there no ++?
            }
        }
    }
    count
}
