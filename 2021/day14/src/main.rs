use std::fs; 
use std::collections::HashMap;

fn main() {
    let file = fs::read_to_string("input14.txt")
        .expect("Could not read file");

    let lines: Vec<&str> = file.split("\n").collect();

    let starting_string = lines[0];
    let mut mapped_sub: HashMap<&str,Pair> = lines[2..].iter().map(|x| Pair::from(x)).collect();
    //println!("{}",mapped_sub)

    let initial_pairs: Vec<String> = (1..starting_string.chars().count()).map(|x| format!("{}{}",&starting_string[x-1..x],&starting_string[x..x+1])).collect();
    //println!("{:?}",initial_pairs);
    for e in initial_pairs {
        mapped_sub.get_mut(e.as_str()).unwrap().count += 1;
    }

    let n = 10;
    for _ in 0..n {
        for (k,v) in mapped_sub.clone() {
            //mapped_sub.get_mut(k).unwrap().count = 0; // I think i need to fix this line
            mapped_sub.get_mut(v.s1.as_str()).unwrap().count += v.count;
            mapped_sub.get_mut(v.s2.as_str()).unwrap().count += v.count;
        }
    }

    println!("{:?}",mapped_sub)

    // do counts
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