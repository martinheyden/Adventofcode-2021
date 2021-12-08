use aoc_2021::read_input;
use itertools::Itertools;

const ZERO: [usize; 6] = [0, 1, 2, 4, 5, 6];
const ONE: [usize; 2] = [2, 5];
const TWO: [usize; 5] = [0, 2, 3, 4, 6];
const THREE: [usize; 5] = [0, 2, 3, 5, 6];
const FOUR: [usize; 4] = [1, 2, 3, 5];
const FIVE: [usize; 5] = [0, 1, 3, 5, 6];
const SIX: [usize; 6] = [0, 1, 3, 4, 5, 6];
const SEVEN: [usize; 3] = [0, 2, 5];
const EIGHT: [usize; 7] = [0, 1, 2, 3, 4, 5, 6];
const NINE: [usize; 6] = [0, 1, 2, 3, 5, 6];

fn main() {
    println!("{}", problem_a("data/day08.txt"));
    println!("{}", problem_b("data/day08.txt"));
}

//Mean is optimal height
fn problem_a(file_name: &str) -> i64 {
    let str_vec = read_input::read_file_to_string_vec(file_name);
    let mut count = 0;
    let len_vec: [usize; 4] = [2, 3, 4, 7];
    for str in str_vec {
        let mut output = false;
        for seg in str.split_whitespace() {
            if seg == "|" {
                output = true;
            }
            if output && len_vec.contains(&seg.len()) {
                count += 1;
            }
        }
    }
    count
}

fn problem_b(file_name: &str) -> i64 {
    let str_vec = read_input::read_file_to_string_vec(file_name);
    let mut count = 0;
    let mut ans_map = vec![];
    for str in str_vec {
        let mut val = 0;
        let mut output = false;
        let mut info = vec![];
        for seg in str.split_whitespace() {
            if seg == "|" {
                output = true;
                ans_map = find_map(&info);
            } else {
                if output {
                    //Now we should now each letter
                    val = val * 10 + letter(&ans_map, seg);
                } else {
                    info.push(seg);
                }
            }
        }
        count += val;
    }
    count
}

fn letter(ans_map: &Vec<char>, seg: &str) -> i64 {
    let mut nbrs = vec![];
    for c in seg.chars() {
        for i in 0..7 {
            if ans_map[i] == c {
                nbrs.push(i);
            }
        }
    }
    nbrs.sort();
    if compare(&ONE, &nbrs) {
        return 1;
    } else if compare(&TWO, &nbrs) {
        return 2;
    } else if compare(&THREE, &nbrs) {
        return 3;
    } else if compare(&FOUR, &nbrs) {
        return 4;
    } else if compare(&FIVE, &nbrs) {
        return 5;
    } else if compare(&SIX, &nbrs) {
        return 6;
    } else if compare(&SEVEN, &nbrs) {
        return 7;
    } else if compare(&EIGHT, &nbrs) {
        return 8;
    } else if compare(&NINE, &nbrs) {
        return 9;
    } else if compare(&ZERO, &nbrs) {
        return 0;
    } else {
        panic!("could find number");
    }
}

fn compare(arr: &[usize], v: &Vec<usize>) -> bool {
    if arr.len() == v.len() {
        for i in 0..arr.len() {
            if arr[i] != v[i] {
                return false;
            }
        }
        return true;
    }
    false
}

fn find_map(info: &Vec<&str>) -> Vec<char> {
    let letters: Vec<char> = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g'];
    for candidate in letters.iter().permutations(7).unique() {
        if fits(&candidate, info) {
            return candidate.iter().map(|c| **c).collect();
        }
    }
    panic!("should have found candidate!");
}

fn fits(candidate: &Vec<&char>, info: &Vec<&str>) -> bool {
    for nbr in info {
        if !is_valid_digit(candidate, nbr) {
            return false;
        }
    }
    return true;
}

fn is_valid_digit(candidate: &Vec<&char>, nbr: &str) -> bool {
    let active_segments = get_active_segments(candidate, nbr);
    match active_segments.len() {
        2 => {
            return active_segments
                .iter()
                .enumerate()
                .fold(true, |bol, e| ONE[e.0] == *e.1 && bol)
        }
        3 => {
            return active_segments
                .iter()
                .enumerate()
                .fold(true, |bol, e| SEVEN[e.0] == *e.1 && bol)
        }
        4 => {
            return active_segments
                .iter()
                .enumerate()
                .fold(true, |bol, e| FOUR[e.0] == *e.1 && bol)
        }
        5 => {
            return active_segments
                .iter()
                .enumerate()
                .fold(true, |bol, e| TWO[e.0] == *e.1 && bol)
                || active_segments
                    .iter()
                    .enumerate()
                    .fold(true, |bol, e| THREE[e.0] == *e.1 && bol)
                || active_segments
                    .iter()
                    .enumerate()
                    .fold(true, |bol, e| FIVE[e.0] == *e.1 && bol)
        }
        6 => {
            return active_segments
                .iter()
                .enumerate()
                .fold(true, |bol, e| ZERO[e.0] == *e.1 && bol)
                || active_segments
                    .iter()
                    .enumerate()
                    .fold(true, |bol, e| SIX[e.0] == *e.1 && bol)
                || active_segments
                    .iter()
                    .enumerate()
                    .fold(true, |bol, e| NINE[e.0] == *e.1 && bol)
        }
        7 => return true,
        _ => panic!("Weird length"),
    }
}

fn get_active_segments(candidate: &Vec<&char>, nbr: &str) -> Vec<usize> {
    let mut v = Vec::new();
    for i in nbr.chars() {
        for j in 0..7 {
            if i == *candidate[j] {
                v.push(j);
            }
        }
    }
    v.sort();
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(26, problem_a("data/day08_test.txt"))
    }

    #[test]
    fn part2() {
        assert_eq!(61229, problem_b("data/day08_test.txt"))
    }
}
