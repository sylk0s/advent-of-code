use std::fs;

fn main() {
    let file = fs::read_to_string("input.txt")
        .expect("Could not read file");

    let lines: Vec<&str> = file.split("\n").collect();
    println!("Part 1: {}", pt1(&lines));
    println!("Part 2: {}", pt2(&lines));
}

fn pt1(l: &Vec<&str>) -> i32 {

    l.iter().map(|a| a.split(" ").collect()).collect::<Vec<Vec<&str>>>().iter().fold(0, |acc, x| {
        let opposing_pick = RPS::pick_to_bin(x[0]).unwrap();
        let my_pick = RPS::pick_to_bin(x[1]).unwrap();
        acc + (my_pick.score(opposing_pick) as i32)
    })
}

fn pt2(l: &Vec<&str>) -> i32 {
    0
}

#[derive(Clone, Copy)]
#[repr(u8)]
enum RPS {
    ROCK = 1,
    PAPER = 2,
    SCISSORS = 3,
}

impl RPS {
    fn pick_to_bin(pick: &str) -> Option<RPS> {
        return match pick {
            "A" | "X" => Some(RPS::ROCK),
            "B" | "Y" => Some(RPS::PAPER),
            "C" | "Z" => Some(RPS::SCISSORS),
            _ => None,
        }
    }

    fn score(&self, opposing: RPS) -> u8 {
        (*self as u8) + (((*self as u8) + (opposing as u8) + 3) % 3) 
    }
}
