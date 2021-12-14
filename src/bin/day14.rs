use aoc_2021::read_input;
use regex::Regex;
use std::collections::HashMap;

fn read_input(file_name: &str) -> (Vec<char>, HashMap<[char; 2], char>) {
    let re = Regex::new(r"^(\w)(\w) -> (\w)$").unwrap();
    let strings = read_input::read_file_to_string_vec(file_name);
    let init = strings[0].chars().collect::<Vec<char>>();
    let mut rules = HashMap::new();
    for i in 2..strings.len() {
        let c = re.captures(&strings[i]).unwrap();
        rules.insert(
            [c[1].chars().next().unwrap(), c[2].chars().next().unwrap()],
            c[3].chars().next().unwrap(),
        );
    }
    (init, rules)
}

fn one_iterate(
    old: HashMap<[char; 2], usize>,
    rules: &HashMap<[char; 2], char>,
) -> HashMap<[char; 2], usize> {
    let mut new = HashMap::new();
    for (k, v) in old {
        let c = rules.get(&k).unwrap();
        *new.entry([k[0], *c]).or_insert(0) += v;
        *new.entry([*c, k[1]]).or_insert(0) += v;
    }
    new
}

//Keep track of the number of each pair of letters
fn do_n_iters(n: usize, polymer: Vec<char>, rules: &HashMap<[char; 2], char>) -> usize {
    //Initialize map
    let mut par_map = HashMap::new();
    for i in 0..polymer.len() - 1 {
        *par_map.entry([polymer[i], polymer[i + 1]]).or_insert(0) += 1
    }
    for _ in 0..n {
        par_map = one_iterate(par_map, rules);
    }
    let mut count_map = HashMap::new(); //count of each letter
    for (k, v) in par_map {
        // Only cound second letter in each word, otherwise every letter will be counted twice (except first at first iteration)
        *count_map.entry(k[1]).or_insert(0) += v;
    }
    *count_map.entry(polymer[0]).or_insert(0) += 1; //First letter is not counted above
    let mut max = 0;
    let mut min = std::usize::MAX;
    for (_, val) in count_map.iter() {
        if *val > max {
            max = *val;
        }
        if *val < min {
            min = *val;
        }
    }
    max - min
}

fn problem_a(file_name: &str) -> usize {
    let (init, rules) = read_input(file_name);
    do_n_iters(10, init, &rules)
}

fn problem_b(file_name: &str) -> usize {
    let (init, rules) = read_input(file_name);
    do_n_iters(40, init, &rules)
}

fn main() {
    println!("{}", problem_a("data/day14.txt"));
    println!("{}", problem_b("data/day14.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(1588, problem_a("data/day14_test.txt"))
    }

    #[test]
    fn part2() {
        assert_eq!(2188189693529, problem_b("data/day14_test.txt"))
    }
}
