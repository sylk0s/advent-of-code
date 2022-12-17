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

    let state = state_lines.fold(init, |mut acc, line| {
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

    let moves: Vec<(u32, u32, u32)> = file_lines.iter().filter(|line| line.starts_with("move") ).map(|x| parse_instruction(x)).collect();

    println!("Part 1: {}", pt1(moves.clone(), state.clone()));
    println!("Part 2: {}", pt2(moves, state));
}

fn pt1(moves: Vec<(u32, u32, u32)>, state: Vec<Vec<char>>) -> String {
    let fin = moves.iter().fold(state, |acc, m| { update(*m, acc) });
    fin.iter().fold(String::new(), |mut acc, l| { acc.push(*l.first().unwrap()); acc })
}

fn pt2(moves: Vec<(u32, u32, u32)>, state: Vec<Vec<char>>) -> String {
    let fin = moves.iter().fold(state, |acc, m| { update2(*m, acc) });
    fin.iter().fold(String::new(), |mut acc, l| { acc.push(*l.first().unwrap()); acc })
}

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

fn update(inst: (u32, u32, u32), mut state: Vec<Vec<char>>) -> Vec<Vec<char>> {
    for _ in 0..inst.0 {
        let item = state[inst.1 as usize].remove(0);
        state[inst.2 as usize].insert(0, item);
    }
    state
}

fn update2(inst: (u32, u32, u32), mut state: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut move_queue = vec![];
    for _ in 0..inst.0 {
        move_queue.push(state[inst.1 as usize].remove(0));
    }
    for item in move_queue.iter().rev() {
        state[inst.2 as usize].insert(0, *item);
    }
    state
}

// im choosing to do this because I want it to be fully expandable
fn parse_instruction(line: &str) -> (u32, u32, u32) {
    let inst: Vec<u32> = line.split(" ").enumerate().filter(|x| x.0%2 != 0).map(|y| y.1.parse::<u32>().unwrap()).collect();
    (inst[0], inst[1]-1, inst[2]-1)
}
