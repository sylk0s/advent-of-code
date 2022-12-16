use std::fs;
use std::iter::Iterator;

fn main() {
    let file = fs::read_to_string("input.txt")
        .expect("Could not read file");

    let file_lines: Vec<&str> = file.split("\n").collect();

    let state_lines = file_lines.iter().take_while(|line| **line != "");
    let init = state_lines.clone().last().expect("Lines last broke").split("   ").fold(Vec::new(), |mut acc: Vec<Vec<char>>, _|{
        acc.push(Vec::new());
        acc
    });

    let mut state = state_lines.fold(init, |mut acc, line| {
        if line.starts_with(" 1") { return acc; }
        else {
            for item in split_n(line, 4).iter().enumerate() {
                
                if item.1 != &' ' {
                    acc[item.0].push(*item.1);
                }
            }
            return acc
        }
    }); 

    //println!("Part 1: {}", pt1(&lines));
    //println!("Part 2: {}", pt2(&lines));
}
/*
fn pt1(l: Vec<&str>) -> u32 {
    
}

fn pt2(l: Vec<&str>) -> u32 {
    0
}
*/

fn split_n(string: &str, n: u32) -> Vec<char>{
    string.chars().fold((n, vec![vec![]]), |mut acc, x| {
        if acc.0 == 1 {
            acc.1.push(Vec::new());
            acc.0 = n;
        } else {
            let l = acc.1.len()-1;
            acc.1[l].push(x);
            acc.0 -= 1;
        }
        acc
    }).1.iter().map(|a| a[1]).collect()
}
