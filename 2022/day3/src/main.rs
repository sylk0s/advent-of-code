use std::fs;

fn main() {
    let file = fs::read_to_string("input.txt")
        .expect("Could not read file");

    let lines: Vec<&str> = file.split("\n").collect();
    println!("Part 1: {}", pt1(lines.clone()));
    println!("Part 2: {}", pt2(lines));
}

fn pt1(l: Vec<&str>) -> u32 {
    l.iter().map(|x| {
        let index = x.chars().count()/2;
        (x.chars().take(index), x.chars().skip(index))
    }).fold(0, |acc, x| {
        for c in x.0 {
            if x.1.clone().any(|a| a == c ) {
                return acc + char_to_pri(c);
            }
        }
        acc
    })
}

fn pt2(mut l: Vec<&str>) -> u32 {
    let mut counter = 0;
    let mut strings = Vec::new();
    let mut sum = 0;
    while let Some(s) = l.pop() {
        strings.push(s);
        counter += 1;
        if counter == 3 {
            'a: for a in strings[0].chars() {
                for b in strings[1].chars() {
                    if strings[2].chars().any(|c| c == a && c == b) {
                        sum += char_to_pri(a);
                        break 'a;
                    }
                }
            }
            counter = 0;
            strings = Vec::new();
        }
    }
    sum
}

fn char_to_pri(c: char) -> u32 {
    if c.is_uppercase() {
        return c as u32 - 38;
    }
    c as u32 - 96
}

#[cfg(test)]
mod tests {
    #[test]
    fn typecast() {
        assert_eq!(('a' as u32), 97);
        assert_eq!(('A' as u32), 65);
    }

    #[test]
    fn priority() {
        assert_eq!(crate::char_to_pri('a'), 1);
        assert_eq!(crate::char_to_pri('A'), 27);
    }

    #[test]
    fn ormap() {
        assert_eq!("abcdefg".chars().fold(false, |a, b| a || (b == 'e')), true);
    }
}
