use std::fs;

fn main() {
    let file = fs::read_to_string("input5.txt")
        .expect("Could not read file");

    let lines: Vec<&str> = file.split("\n").collect();

    println!("{}",part_one(&lines));
    //println!("{}",part_two(&lines));
}

fn part_one(lines: &Vec<&str>) -> u32 {

    let line_vec = Line::parse_to_flat_line(lines);

    let mut total_count = 0;
    for i in 0..1000 {
        for j in 0..1000 {
            let mut count = 0;
            for line in &line_vec {
                if line.point_exists(i,j) {
                    count += 1;
                }
                if count > 1 {
                    // println!("Found intersection at ({},{})",i,j);
                    total_count += 1;
                    break;
                }
            }
        }
    }
    total_count
}

struct Line {
    x1: u32,
    y1: u32,
    x2: u32,
    y2: u32,
}

impl Line {
    fn parse_to_flat_line(lines: &Vec<&str>) -> Vec<Line> {

        let mut results = Vec::new();

        for line in lines {
            let temp_vec: Vec<&str> = line.split(" -> ").collect();
            let coord1: Vec<&str> = temp_vec[0].split(',').collect();
            let coord2: Vec<&str> = temp_vec[1].split(',').collect();

            let x1 = coord1[0].trim().parse::<u32>().expect("Parsing error");
            let y1 = coord1[1].trim().parse::<u32>().expect("Parsing error");
            let x2 = coord2[0].trim().parse::<u32>().expect("Parsing error");
            let y2 = coord2[1].trim().parse::<u32>().expect("Parsing error");

            if x1 == x2 || y1 == y2 {
                results.push(Line {x1:x1,y1:y1,x2:x2,y2:y2});
            }
        }
        results
    }
}

impl Line { // doesnt wanna work unless i do this grrrr
    fn point_exists(&self,px:u32,py:u32) -> bool {
        if self.x1 == self.x2 && self.x1 == px {
            return is_between(self.y1,self.y2,py);
        }
        if self.y1 == self.y2 && self.y1 == py {
            return is_between(self.x1,self.x2,px);
        }
        false
    }
}

fn is_between(n1:u32, n2:u32, p:u32) -> bool {
    (n1 >= p && p >= n2) || (n2 >= p && p >= n1)
}