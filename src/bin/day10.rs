use aoc_2021::read_input;

fn main() {
    println!("{}", problem_a("data/day10.txt"));
    println!("{}", problem_b("data/day10.txt"));
}

fn opening(ch: char) -> bool {
    match ch {
        '(' => true,
        '[' => true,
        '{' => true,
        '<' => true,
        _ => false,
    }
}

fn score(ch: char) -> i64 {
    match ch {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("expected closing"),
    }
}

fn score_b(ch: char) -> i64 {
    match ch {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => panic!("expected closing"),
    }
}

fn switch(ch: char) -> char {
    match ch {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => panic!("Couldnt switch"),
    }
}

enum Result {
    Invalid(i64),
    Incomplete,
}

fn parse_line(line: &str, queue: &mut Vec<char>) -> Result {
    queue.clear();
    for ch in line.chars() {
        if opening(ch) {
            queue.push(ch);
        } else {
            let opening = queue.pop().unwrap();
            if switch(opening) != ch {
                return Result::Invalid(score(ch));
            }
        }
    }
    Result::Incomplete
}

fn problem_a(file_name: &str) -> i64 {
    let str_vec = read_input::read_file_to_string_vec(file_name);
    let mut count = 0;
    let mut queue = Vec::new();
    for string in str_vec {
        match parse_line(&string, &mut queue) {
            Result::Invalid(i) => count += i,
            _ => (),
        };
    }
    count
}

fn problem_b(file_name: &str) -> i64 {
    let str_vec = read_input::read_file_to_string_vec(file_name);
    let mut score_vec = Vec::new();
    let mut queue = Vec::new();
    for string in str_vec {
        match parse_line(&string, &mut queue) {
            Result::Invalid(_) => (),
            Result::Incomplete => {
                let mut score = 0;
                for ch in queue.iter().rev() {
                    score = score * 5 + score_b(switch(*ch));
                }
                score_vec.push(score);
            }
        };
    }
    score_vec.sort();
    score_vec[score_vec.len() / 2]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(26397, problem_a("data/day10_test.txt"))
    }

    #[test]
    fn part2() {
        assert_eq!(288957, problem_b("data/day10_test.txt"))
    }
}
