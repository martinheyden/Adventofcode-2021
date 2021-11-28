#[path = "../utils.rs"]
mod utils;

use regex::Regex;

fn main() {
    println!("{:?}",problem_a("data/2020day2.txt"));
    println!("{:?}",problem_b("data/2020day2.txt"));
}

fn problem_a(filename: &str) ->usize {
    let data_vec = utils::read_file_to_string_array(filename);
    let re = Regex::new(r"^(\d+)-(\d+)+\s(\w):\s(\w+)$").unwrap();
    data_vec.iter().map(|line| re.captures(line).unwrap())
        .filter(|c| {
            let matches = c[4].matches(&c[3]).count();
            matches>= c[1].parse().unwrap() && matches <= c[2].parse().unwrap()
        })
        .count()
}

fn problem_b(filename: &str) ->usize {
    let data_vec = utils::read_file_to_string_array(filename);
    let re = Regex::new(r"^(\d+)-(\d+)+\s(\w):\s(\w+)$").unwrap();
    data_vec.iter().map(|line| re.captures(line).unwrap())
        .filter(|c| {
            let b1 = c[4].chars().nth(c[1].parse::<usize>().unwrap()-1).unwrap() == c[3].parse::<char>().unwrap();
            let b2 = c[4].chars().nth(c[2].parse::<usize>().unwrap()-1).unwrap() == c[3].chars().next().unwrap();
            if (b1&&b2) || !(b1||b2) {
                false
            } else {
                true
            }
        })
        .count()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() { 
        assert_eq!(2,problem_a("data/2020day2_test.txt"))
    }


}