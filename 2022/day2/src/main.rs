use std::fs;

fn main() {
    let file = fs::read_to_string("input.txt")
        .expect("Could not read file");

    let lines: Vec<&str> = file.split("\n").collect();
    println!("Part 1: {}", pt1(&lines));
    println!("Part 2: {}", pt2(&lines));
}

fn pt1(l: &Vec<&str>) -> i32 {
    l.iter().map(|a| a.split(" ").collect()).collect::<Vec<Vec<&str>>>().iter().fold(0, |acc, x| {
        if x[0] == "" {
            return acc;
        }
        let opposing_pick = RPS::pick_to_bin(x[0]).unwrap();
        let my_pick = RPS::pick_to_bin(x[1]).unwrap();
        acc + (my_pick.score(opposing_pick) as i32)
    })
}

fn pt2(l: &Vec<&str>) -> i32 {
    l.iter().map(|a| a.split(" ").collect()).collect::<Vec<Vec<&str>>>().iter().fold(0, |acc, x| {
        if x[0] == "" {
            return acc;
        }
        let opposing_pick = RPS::pick_to_bin(x[0]).unwrap();
        acc + opposing_pick.get_play(x[1]) as i32
    })
}

#[derive(Clone, Copy, PartialEq)]
#[repr(u8)]
enum RPS {
    ROCK = 1,
    PAPER = 2,
    SCISSORS = 3,
}

impl RPS {
    fn pick_to_bin(pick: &str) -> Option<RPS> {
        return match pick {
            "A" | "X" => Some(RPS::ROCK),
            "B" | "Y" => Some(RPS::PAPER),
            "C" | "Z" => Some(RPS::SCISSORS),
            _ => None,
        }
    }

    fn score(&self, opposing: RPS) -> u8 {
        if *self == opposing {
            return 3 + (*self as u8);
        } else if (*self as u8) == (opposing as u8) % 3 + 1 {
            return *self as u8 + 6
        }
        *self as u8
    }

    fn get_play(&self, res: &str) -> u8 {
        return match res {
            "Z" => (*self as u8) % 3 + 7,
            "Y" => (*self as u8) + 3,
            "X" => {
                if (*self as u8)-1 == 0 { return 3; } else {
                    return (*self as u8)-1;
                }
            },
            _ => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::RPS;

    #[test]
    fn score_for_play() {
        assert_eq!(RPS::ROCK.get_play("X"), 3);
        assert_eq!(RPS::ROCK.get_play("Y"), 4);
        assert_eq!(RPS::ROCK.get_play("Z"), 8);
    }

    #[test]
    fn scoring() {
        assert_eq!(RPS::ROCK.score(RPS::PAPER), 1);
        assert_eq!(RPS::PAPER.score(RPS::ROCK), 8);
        assert_eq!(RPS::PAPER.score(RPS::PAPER), 5);
        assert_eq!(RPS::SCISSORS.score(RPS::ROCK), 3);
        assert_eq!(RPS::ROCK.score(RPS::SCISSORS), 7);
        assert_eq!(RPS::SCISSORS.score(RPS::PAPER), 9);
        assert_eq!(RPS::PAPER.score(RPS::SCISSORS), 2);
    }
}
