use aoc_2021::read_input;
use num::signum;
use regex::Regex;
use std::collections::HashMap;

fn main() {
    println!("{}", problem_a("data/day05.txt"));
    println!("{}", problem_b("data/day05.txt"));
}

fn problem_a(file_name: &str) -> i64 {
    solve_problem(file_name, false)
}

fn problem_b(file_name: &str) -> i64 {
    solve_problem(file_name, true)
}

fn solve_problem(file_name: &str, count_diags: bool) -> i64 {
    let data_vec = read_input::read_file_to_string_vec(file_name);
    let mut count_map: HashMap<(i64, i64), i64> = HashMap::new();
    let re = Regex::new(r"^(\d+),(\d+)\s->\s(\d+),(\d+)$").unwrap();
    for line in data_vec {
        let caps = re.captures(&line);
        let params = (1..=4)
            .map(|x| caps.as_ref().unwrap()[x].parse::<i64>().unwrap())
            .collect::<Vec<i64>>(); //first element is entire string
        let (x0, y0, x1, y1) = (params[0], params[1], params[2], params[3]);
        let dir = (signum(x1 - x0), signum(y1 - y0));
        let mut x = x0;
        let mut y = y0;
        //If diagonals are to be counted, or not diagonal
        if count_diags || !(x0 != x1 && y0 != y1) {
            //Count the number of steps to be taken
            for _i in 0..=std::cmp::max((x0 - x1).abs(), (y0 - y1).abs()) {
                let count = count_map.entry((x, y)).or_insert(0);
                *count += 1;
                x = x + dir.0;
                y = y + dir.1;
            }
        }
    }
    let mut count = 0;
    for (_, v) in count_map {
        if v >= 2 {
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(5, problem_a("data/day05_test.txt"))
    }

    #[test]
    fn part2() {
        assert_eq!(12, problem_b("data/day05_test.txt"))
    }
}
