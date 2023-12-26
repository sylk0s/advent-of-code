use std::{fs, collections::HashMap, hash::Hash};
use regex::Regex;

fn main() {
    let file = fs::read_to_string("input.txt")
    .expect("Could not read file");

    let mut max_map = HashMap::new();
    max_map.insert("blue", 14);
    max_map.insert("green", 13);
    max_map.insert("red", 12);

    let lines: Vec<String> = file.split("\n").map(|l| l.to_string()).collect();

    println!("PT1: {}", pt1(lines.clone(), max_map.clone()));
}

fn pt1(lines: Vec<String>, max_map: HashMap<&str, i32>) -> i32 {

    // lines.iter().enumerate().fold(0, |acc, (i, l)| {
    //     let line: String = l.split("").skip(8).collect();
    //     let mut map: HashMap<&str, i32> = HashMap::new();

    //     let seperator = Regex::new(r"([,;])").expect("invalid regex");
    //     seperator.split(&line).into_iter()
    //     .for_each(|entry| {
    //         let kv = entry.split(" ").skip(1).collect::<Vec<_>>();
    //         if kv.len() != 2 {
    //             println!("{:?}", kv);
    //         }
    //         put_or_add(&mut map, kv[1], kv[0].parse().unwrap());
    //     });

    //     if is_valid_map(map, &max_map) {
    //         acc + i as i32
    //     } else {
    //         acc
    //     }
    // })

    lines.iter().enumerate().fold(0, |acc, (i, l)| {
        let line: String = l.split("").skip(8).collect();
        let mut map: HashMap<&str, i32> = HashMap::new();

        let seperator = Regex::new(r"([,;])").expect("invalid regex");
        seperator.split(&line).into_iter()
        .for_each(|entry| {
            let kv = entry.split(" ").skip(1).collect::<Vec<_>>();
            if kv.len() != 2 {
                println!("{:?}", kv);
            }
            put_or_add(&mut map, kv[1], kv[0].parse().unwrap());
        });

        if is_valid_map(map, &max_map) {
            acc + i as i32
        } else {
            acc
        }
    })
    })
}

fn is_valid_map(map: HashMap<&str, i32>, max_map: &HashMap<&str, i32>) -> bool {
    map.iter().all(|(k, v)| {
        match max_map.get(k) {
            Some(max) => v <= max,
            None => false,
    }})
}

fn put_or_add<T, U>(map: &mut HashMap<T, U>, key: T, value: U)
where T: Eq + Hash, U: std::ops::AddAssign<U> {
    match map.get_mut(&key) {
        Some(v) => *v += value,
        None => {map.insert(key, value);},
    }
}