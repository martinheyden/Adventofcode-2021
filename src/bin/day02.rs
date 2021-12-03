use aoc_2021::read_input;
use regex::Regex;

enum Coordinate {
    TwoDim {
        depth: i64,
        position: i64,
    },
    ThreeDim {
        depth: i64,
        angle: i64,
        position: i64,
    },
}

impl Coordinate {
    fn forward(&mut self, arg: i64) {
        match self {
            Coordinate::TwoDim { position, .. } => *position += arg,
            Coordinate::ThreeDim {
                depth,
                angle,
                position,
            } => {
                *position += arg;
                *depth += arg * *angle;
            }
        };
    }

    fn down(&mut self, arg: i64) {
        match self {
            Coordinate::TwoDim { depth, .. } => *depth += arg,
            Coordinate::ThreeDim { angle, .. } => *angle += arg,
        };
    }

    fn up(&mut self, arg: i64) {
        self.down(-arg);
    }

    fn product(&self) -> i64 {
        match self {
            Coordinate::TwoDim { depth, position } => *depth * *position,
            Coordinate::ThreeDim {
                depth, position, ..
            } => *depth * *position,
        }
    }
}

fn main() {
    println!("{}", problem_a("data/day02.txt"));
    println!("{}", problem_b("data/day02.txt"));
}

fn navigate(filename: &str, mut coord: Coordinate) -> i64 {
    let data_vec = read_input::read_file_to_string_vec(filename);
    let re = Regex::new(r"^(\w+)\s(\d+)$").unwrap(); //A bit overkill maybe...
    for line in data_vec.iter() {
        let c = re.captures(line).unwrap();
        let arg = c[2].parse::<i64>().unwrap();
        match &c[1] {
            "forward" => coord.forward(arg),
            "down" => coord.down(arg),
            "up" => coord.up(arg),
            _ => panic!(),
        }
    }
    coord.product()
}

fn problem_a(filename: &str) -> i64 {
    navigate(
        filename,
        Coordinate::TwoDim {
            depth: 0,
            position: 0,
        },
    )
}

fn problem_b(filename: &str) -> i64 {
    navigate(
        filename,
        Coordinate::ThreeDim {
            depth: 0,
            angle: 0,
            position: 0,
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(150, problem_a("data/day02_test.txt"))
    }

    #[test]
    fn part2() {
        assert_eq!(900, problem_b("data/day02_test.txt"))
    }
}
