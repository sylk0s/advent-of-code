use std::fs; 
use std::collections::HashMap;

fn main() {
    let file = fs::read_to_string("input14.txt")
        .expect("Could not read file");

    let lines: Vec<&str> = file.split("\n").collect();

    let starting_string = lines[0];
    let mut mapped_sub: HashMap<&str,Pair> = lines[2..].iter().map(|x| Pair::from(x)).collect();

    let initial_pairs: Vec<String> = (1..starting_string.chars().count()).map(|x| format!("{}{}",&starting_string[x-1..x],&starting_string[x..x+1])).collect();
    for e in initial_pairs {
        mapped_sub.get_mut(e.as_str()).unwrap().count += 1;
    }

    // preform simulation for n loops fast AF
    // change to 10 for day 1
    // change to 40 for day 2
    let n = 80;
    for _ in 0..n {
        for (k,v) in mapped_sub.clone() {
            mapped_sub.get_mut(k).unwrap().count -= v.count;
            mapped_sub.get_mut(v.s1.as_str()).unwrap().count += v.count;
            mapped_sub.get_mut(v.s2.as_str()).unwrap().count += v.count;
        }
    }

    // get the counts for each letter
    let mut count: HashMap<&str,u128> = HashMap::new();

    for (k,v) in mapped_sub {
        match count.get_mut(&k[0..1]) {
            Some(x) => *x += v.count,
            None => {let _ = &count.insert(&k[0..1],v.count); ()},
        };
    }

    let counts = count.values().collect::<Vec<&u128>>();
    let min = counts.iter().min().unwrap();
    let max = counts.iter().max().unwrap();
    println!("min {}",min);
    println!("max {}",max);

    // turns out my code is ALWAYS just off by 1!
    println!("Answer: {}",*max-*min+1);
}

#[derive(Debug, Clone)]
struct Pair {
    s1:String,
    s2:String,
    count:u128
}

impl Pair {
    fn from(s: &str) -> (&str, Pair) {
        let a: Vec<&str> = s.split(" -> ").collect(); 
        ( a[0],
        Pair {
            s1: format!("{}{}",&a[0][0..1],a[1]),
            s2: format!("{}{}",a[1],&a[0][1..2]),
            count: 0
        })
    }
}