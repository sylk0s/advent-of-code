use std::fs;
use std::cmp;

fn main() {
    let file = fs::read_to_string("input.txt")
        .expect("Could not read file");

    let lines: Vec<&str> = file.split("\n").collect();
    println!("Part 1: {}", pt1(&lines));
    println!("Part 2: {}", pt2(&lines));
}

fn pt1(l: &Vec<&str>) -> i32 {
    l.iter().fold((0,0), |acc: (i32, i32), x| {
        if let Ok(num) = x.parse::<i32>() {
            let new: i32 = acc.1 + num;
            return (cmp::max(new, acc.0), new);
        } else {
            return (acc.0, 0);
        }
    }).0
}

fn pt2(l: &Vec<&str>) -> i32 {
    let mut top = l.iter().fold((Vec::new(), 0), |acc: (Vec<i32>, i32), x| {
        if let Ok(num) = x.parse::<i32>() {
            return (acc.0, acc.1 + num);
        } else {
            let mut tmp_list = acc.0.clone();
            tmp_list.push(acc.1);
            return (tmp_list, 0);
        }
    }).0;
    top.sort_by(|a, b| b.cmp(a));
    top[0] + top[1] + top[2]
}
