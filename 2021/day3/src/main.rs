use std::fs;

fn main() {
    let file = fs::read_to_string("input3.txt")
        .expect("Could not read file");

    let lines: Vec<&str> = file.split("\n").collect();
    //let lines2: Vec<&str> = file.split("\n").collect();

    println!("{}",part_one(lines));
    //println!("{}",part_two(lines2));
}

fn part_one(lines: Vec<&str>) -> u32 {
    let mut gamma = 0;
    let line_len = lines[0].len();
    let lines_len = lines.len();
    let mut totals = vec![0; line_len];

    for line in lines {
        let bin = u32::from_str_radix(line.trim(), 2).unwrap();
        for n in 0..line_len {
            if bin & 1<<n != 0 {
                totals[n] += 1;
            }
        }
    }

    for n in 0..line_len {
        if totals[n] > lines_len/2 {
            gamma += 1<<n;
        }
    }

    println!("{}",gamma);
    println!("{}",gamma^1<<line_len);
    println!("{}",!0b0110110111100);
    println!("{}",0b0110110111100);
    println!("{}",0b1001001000011);

    // wrong -> println!("{}",3516*4675);


    gamma*(gamma^1<<line_len-1)
}

//fn part_two() {
//    return
//}