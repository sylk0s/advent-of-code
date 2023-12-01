use std::fs;

const NAMES: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

fn replace_with_digits(mut line: String) -> String {

    // cursed but works for overlapping nums, may not actually work for larger nums
    for i in 1..10 {
        while let Some(val) = line.find(NAMES[i-1]) {
            let range = val+1..val+2;
            line.replace_range(range, &i.to_string());
        }
    }
    line
}

fn main() {
    println!("{}", replace_with_digits("eightwothree".to_string()));

    let file = fs::read_to_string("input.txt")
    .expect("Could not read file");

    let lines: Vec<String> = file.split("\n").map(|l| l.to_string()).collect();

    // Uncomment for pt2 :)
    let lines: Vec<String> = lines.iter().map(|line| {
        replace_with_digits(line.clone())
    }).collect();

    let mut count = 0;
    for line in lines {
        let chars = line.chars();

        let first = chars.clone().into_iter().find(|c| c.is_digit(10)).expect("Failed to find first digit");
        let last = chars.into_iter().rfind(|c| c.is_digit(10)).expect("Failed to find last digit");

        println!("{} {}", first, last);

        count += (first.to_digit(10).unwrap()) * 10 + last.to_digit(10).unwrap();
    }

    println!("Count: {}", count);
}