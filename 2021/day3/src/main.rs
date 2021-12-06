use std::fs;

fn main() {
    let file = fs::read_to_string("input3.txt")
        .expect("Could not read file");

    let lines: Vec<&str> = file.split("\n").collect();
    //let lines2: Vec<&str> = file.split("\n").collect();

    println!("{}",part_one(&lines));
    //println!("{}",part_two(lines2));
}

fn part_one(lines: &Vec<&str>) -> u32 {
    let mut gamma = 0;
    let line_length = lines[0].len();
    let mut totals = vec![0; line_length];

    for line in lines {
        let bin = u32::from_str_radix(line.trim(), 2).unwrap();
        for n in 0..line_length {
            if bin & 1<<n != 0 {
                totals[n] += 1;
            }
        }
    }

    for n in 0..line_length {
        if totals[n] > lines.len()/2 {
            gamma += 1<<n;
        }
    }

    gamma*(gamma^0b111111111111)
}

//fn part_two() {
//    return
//}