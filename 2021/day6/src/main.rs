use std::fs;

fn main() {
    let file = fs::read_to_string("input6.txt")
        .expect("Could not read file");

    let lines: Vec<&str> = file.split("\n").collect();

    //too high
    println!("{}",part_one(&lines));
    //println!("{}",part_two(&lines));
}

fn part_one(lines: &Vec<&str>) -> u32 {
    // init groups
    let mut groups = [
        FishGroup {count:0,timer:6},
        FishGroup {count:0,timer:5},
        FishGroup {count:0,timer:4},
        FishGroup {count:0,timer:3},
        FishGroup {count:0,timer:2},
        FishGroup {count:0,timer:1},
    ];

    let mut new_fish = 0;
    let init_fish: Vec<&str> = lines[0].split(",").collect();
    for e in init_fish {
        let fish = e.trim().parse::<u32>().expect("Parsing error");
        for n in 0..groups.len() {
            if fish == groups[n].timer {
                groups[n].count += 1;
            }
        }
    }

    for _ in 0..80 {
        for n in 0..groups.len() {
                groups[n].timer -= 1;
            if groups[n].timer == 0 {
                let temp = new_fish;
                new_fish = groups[n].count;
                groups[n].count += temp;
                groups[n].timer = 6;
            }
        }
    }
    // decrement groups
    // create children
    let mut total_fish = 0;

    for group in &groups {
        total_fish += group.count;
    }
    total_fish + new_fish
}

struct FishGroup {
    count: u32,
    timer: u32,
}