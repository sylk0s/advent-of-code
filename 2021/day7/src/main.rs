use std::fs; 

fn main() {
    let file = fs::read_to_string("input7.txt")
        .expect("Could not read file");

    let positions: Vec<&str> = file.split(",").collect();

    println!("{}",part_one(&positions));
    println!("{}",part_two(&positions));
}

fn part_one(lines: &Vec<&str>) -> i32 {
    let mut ilines = parse_vec_to_int(lines);
    ilines.sort();
    let m1 = ilines[(ilines.len()/2)];
    let m2 = ilines[(ilines.len()/2)-1];
    let center = (m1 + m2) / 2;
    let mut cost = 0;
    for i in ilines {
        cost += (i - center).abs();
    }
    cost
}

// doesn't break; logic issue atm
fn part_two(lines: &Vec<&str>) -> i32 {
    let ilines = parse_vec_to_int(lines);
    let mut sum = 0; // need to find a better way to sum stuff later :(
    for i in &ilines {
        sum += i;
    }
    let center = sum / (ilines.len() as i32);
    let mut cost = 0;
    for i in ilines {
        for j in 0..(i-center).abs() {
            cost += j;
        }
    }
    // low -> 101873762
    cost
}

fn parse_vec_to_int(lines: &Vec<&str>) -> Vec<i32> {
    let mut v: Vec<i32> = Vec::new();
    for l in lines {
        v.push(l.trim().parse::<i32>().expect("Parsing Error"));
    }
    v
}