
use aoc_2021::read_input as read_input;
use std::collections::VecDeque;

const HORIZON: usize = 3; // No variable length arrays...

fn main() {
    println!("{}",problem_a("data/day01.txt"));
    println!("{}",problem_b("data/day01.txt"));
}

fn problem_a(file_name: &str) -> i64 {
    calc_larger(&read_input::read_file_to_int_array(file_name))    
}

fn problem_b(file_name: &str) -> i64 {
    calc_larger(
        &moving_average(&read_input::read_file_to_int_array(file_name))[2..].to_vec())
}

fn calc_larger(v: &Vec<i64>) -> i64 {
    v.iter()
    .fold((-1,-1), |acc,el| 
        if *el>acc.0 {
            (*el,acc.1+1)
        } else {
            (*el,acc.1)
        }

    ).1
}

fn moving_average(v: &Vec<i64>,) -> Vec<i64> {
    let mut deq = VecDeque::from(vec![0;HORIZON-1]);
    let mut acc = 0;
   // let mut res = 0;
    v.iter()
    .map(|x| {
        let res = *x + acc;
        deq.push_back(*x);
        acc = acc + *x - deq.pop_front().unwrap();
        res}
    ).collect()
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() { 
        assert_eq!(7,problem_a("data/day01_test.txt"))
    }

    #[test]
    fn part2() { 
        assert_eq!(5,problem_b("data/day01_test.txt"))
    }

}