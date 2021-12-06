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

// doesnt work yet
fn part_two(lines: &Vec<&str>) -> u32 {
    let bits = find_common_bits(&lines);

    let oxygen = last_meeting_criteria(lines, bits);
    let co2 = last_meeting_criteria(lines, bits^0b111111111111);

    //too high
    oxygen*co2
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

fn last_meeting_criteria(lines: &Vec<&str>, bits: u32) -> u32 {
    let mut possible_solutions = lines.clone();
    let line_length = lines[0].len();

    for i in 0..line_length  {
        let mut remove_queue = Vec::<usize>::new();
        for n in 0..possible_solutions.len() {
            let bin = u32::from_str_radix(possible_solutions[n].trim(), 2).unwrap();
            if bin & 1<<(line_length-i-1) & bits == 0 {
                if possible_solutions.len()-remove_queue.len() == 1 {
                    return u32::from_str_radix(possible_solutions[n].trim(), 2).unwrap();
                }
                remove_queue.push(n);
            }
        }
        for item in remove_queue {
            possible_solutions.remove(item);
        }
    }
    u32::from_str_radix(possible_solutions[0].trim(), 2).unwrap()
}