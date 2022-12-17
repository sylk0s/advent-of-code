use std::fs;
use std::str::Chars;
use std::cmp;

fn main() {
    let file = fs::read_to_string("input.txt")
        .expect("Could not read file");

    let chars = file.chars();
    println!("Part 1: {}", consecutive_unique(chars.clone(), 4));
    println!("Part 2: {}", consecutive_unique(chars, 14));
}

fn consecutive_unique(chars: Chars, m: usize) -> usize {
    let n = m-1;
    let mut cache: Vec<char> = chars.clone().take(n).collect();
    let mut rest = chars.skip(n);
    let mut i = n;
    loop {
         i += 1;
         let first = rest.next().unwrap();
         let mut testset = cache.clone();
         testset.push(first);
         if unique(testset) {
            break;
         }
         cache.remove(0);
         cache.push(first);
    }
    i
}


fn unique<T: cmp::PartialEq>(mut v: Vec<T>) -> bool {
    for _ in 0..v.len() {
        let aaa = v.pop().unwrap();
        if v.contains(&aaa) {
            return false;
        }
    }
    true
}
