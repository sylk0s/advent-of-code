use std::fs;

fn main() {
    let file = fs::read_to_string("input3.txt")
        .expect("Could not read file");

    let lines: Vec<&str> = file.split("\n").collect();

    println!("{}",part_one(&lines));
    println!("{}",part_two(&lines));
}

fn part_one(lines: &Vec<&str>) -> u32 {
    let gamma = find_common_bits(lines);
    gamma*(gamma^0b111111111111)
}

fn part_two(lines: &Vec<&str>) -> u32 {
    let bits = find_common_bits();
    let mut possible_solutions = lines.clone();

    for i in 0..lines[0].len()  {
        for line in possible_solutions {
            if line
        }
    }

    //110110110111
    //001001000010

    //110110111100
    //001001000011

    //0b110110110111*0b001001000010
}

fn find_common_bits(lines: &Vec<&str>) -> u32 {
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

    let mut common_bits = 0;
    
    for n in 0..line_length {
        if totals[n] > lines.len()/2 {
            common_bits += 1<<n;
        }
    }

    common_bits
}