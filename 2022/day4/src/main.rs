use std::fs;

fn main() {
    let file = fs::read_to_string("input.txt")
        .expect("Could not read file");

    let lines: Vec<&str> = file.split("\n").collect();
    println!("Part 1: {}", pt1(&lines));
    println!("Part 2: {}", pt2(&lines));
}

// ((r1, r2), (l1, l2))
fn pt1(l: &Vec<&str>) -> u32 {
    l.iter().filter(|s| **s != "").map(|x| x.split(",").collect()).fold(0, |acc, y: Vec<&str>| {
        let r1: Vec<i32> = y[0].split("-").map(|p| p.parse::<i32>().unwrap()).collect();
        let r2: Vec<i32> = y[1].split("-").map(|p| p.parse::<i32>().unwrap()).collect();
        if r1[0] > r2[0] && r1[1] < r2[1] ||
            r1[0] < r2[0] && r1[1] > r2[1] {
            return acc + 1;
        }
        acc
    })
}

fn pt2(l: &Vec<&str>) -> u32 {
    0
}
