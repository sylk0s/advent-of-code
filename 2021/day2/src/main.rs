use std::fs;

fn main() {
    let file = fs::read_to_string("input2.txt")
        .expect("Could not read file");

    let lines: Vec<&str> = file.split("\n").collect();
    let lines2: Vec<&str> = file.split("\n").collect();

    println!("{}",part_one(lines));
    println!("{}",part_two(lines2));
}

fn part_one(lines: Vec<&str>) -> i32 {
    let mut x = 0;
    let mut y = 0;

    for line in lines {
        let instruction = line.split(' ').collect::<Vec<&str>>();
        if instruction[0] == "forward" {
            x = x+instruction[1].trim().parse::<i32>().expect("Parsing error");
        }
        if instruction[0] == "up" {
            y = y-instruction[1].trim().parse::<i32>().expect("Parsing error");
        }
        if instruction[0] == "down" {
            y = y+instruction[1].trim().parse::<i32>().expect("Parsing error");
        }
    }
    println!("{}, {}",x,y);
    x*y
}

fn part_two(lines: Vec<&str>) -> i32 {
    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;

    for line in lines {
        let instruction = line.split(' ').collect::<Vec<&str>>();
        let value = instruction[1].trim().parse::<i32>().expect("Parsing error");
        if instruction[0] == "forward" {
            x = x + value;
            y = y + aim * value;
        }
        if instruction[0] == "up" {
            aim = aim - value;
        }
        if instruction[0] == "down" {
            aim = aim + value;
        }
    }
    x*y
}