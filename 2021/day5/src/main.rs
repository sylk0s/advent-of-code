use std::fs;

fn main() {
    let file = fs::read_to_string("input4.txt")
        .expect("Could not read file");

    let lines: Vec<&str> = file.split("\n").collect();

    println!("{}",part_one(&lines));
    println!("{}",part_two(&lines));
}

fn part_one(lines: &Vec<&str>) {

}

struct Line {
    x1: u32,
    y1: u32,
    x2: u32,
    y2: u32,
}

impl Line {
    fn parse_to_flat_line(lines: &Vec<&str>) -> Vec<Line> {

        let results = Vec::new();

        for line in lines {
            let temp_vec = line.split(' -> ').collect();
            let coord1 = temp_vec[0].split(',').collect();
            let coord2 = temp_vec[1].split(',').collect();

            let x1 = coord1[0].parse();
            let y1 = coord1[1].parse();
            let x2 = coord2[0].parse();
            let y2 = coord2[1].parse();

            if x1 == x2 || y1 == y2 {
                results.push(Line {x1:x1,y1:y1,x2:x2,y2:y2});
            }
        }
        results
    }

    fn intersects(&self, line: &Line) -> bool {
        
    }
}