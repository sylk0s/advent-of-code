use std::fs;
use std::collections::HashMap;

fn main() {
    let file = fs::read_to_string("input.txt")
        .expect("Could not read file");

    let lines: Vec<&str> = file.split("\n").collect();
    let mut dirs: HashMap<String, Directory> = HashMap::new();
    let mut dir = "";
    for line in lines {
        if !line.starts_with("$ cd ..") && line != "" {
            if line.starts_with("$ cd") {
                dir = line.split(" ").skip(2).next().unwrap();
                dirs.insert(dir.to_string(), Directory{ files:vec![], dirs:vec![] }); // new dir
            } else if line.starts_with("dir") {
                dirs.entry(dir.to_string()).and_modify(|e| (*e).dirs.push(line.split(" ").skip(1).next().unwrap().to_string()));
            } else if !line.starts_with("$ ls") {
                dirs.entry(dir.to_string()).and_modify(|e| (*e).files.push(line.split(" ").next().unwrap().parse::<u32>().expect("parse borkn"))); // size value
            }
        }
    }

    //println!("{:?}", dirs);
    println!("Part 1: {}", pt1(dirs));
    //println!("Part 2: {}", pt2(&lines));
}

fn pt1(hash: HashMap<String, Directory>) -> u32 {
    let mut dir_sizes = Vec::new();
    for (name, _) in &hash {
        if let Some(size) = rec_size(name.to_string(), &hash) {
            dir_sizes.push(size);
        }
    }
    dir_sizes.iter().sum()
}
/*
fn pt2() -> u32 {

}
*/

#[derive(Debug, Clone)]
struct Directory {
    files: Vec<u32>,
    dirs: Vec<String>,
}

fn rec_size(key: String, hash: &HashMap<String, Directory>) -> Option<u32> {
    let directory = hash.get(&key).unwrap();
    let mut sum = 0;
    for sub in &directory.dirs {
        if let Some(val) = rec_size(sub.to_string(), &hash) {
            sum += val;
        } else {
            return None;
        }
    }
    let total = sum + directory.files.iter().sum::<u32>();
    if total > 100000 {
        return None;
    }
    Some(total)
}
