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

const MASKS: [&[usize]; 10] = [
    &ZERO, &ONE, &TWO, &THREE, &FOUR, &FIVE, &SIX, &SEVEN, &EIGHT, &NINE,
];

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

fn problem_b(file_name: &str) -> usize {
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

fn letter(ans_map: &Vec<char>, seg: &str) -> usize {
    let mut nbrs = vec![];
    for c in seg.chars() {
        for i in 0..7 {
            if ans_map[i] == c {
                nbrs.push(i);
            }
        }
    }
    nbrs.sort();
    for i in 0..10 {
        if compare(MASKS[i], &nbrs) {
            return i;
        }
    }
    panic!("Could find number");
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
        2 => return check_match(&active_segments, &ONE),
        3 => return check_match(&active_segments, &SEVEN),
        4 => return check_match(&active_segments, &FOUR),
        5 => {
            return check_match(&active_segments, &TWO)
                || check_match(&active_segments, &THREE)
                || check_match(&active_segments, &FIVE)
        }
        6 => {
            return check_match(&active_segments, &ZERO)
                || check_match(&active_segments, &SIX)
                || check_match(&active_segments, &NINE)
        }
        7 => return true, //corresponds to eight
        _ => panic!("Weird length"),
    }
}

//Check if the active segments matches the mask
fn check_match(active_segments: &Vec<usize>, mask: &[usize]) -> bool {
    active_segments
        .iter()
        .enumerate()
        .fold(true, |bol, e| mask[e.0] == *e.1 && bol)
}

//Gets the active segments (0 through 6) for nbr based on the candidate map
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
