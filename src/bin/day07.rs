use aoc_2021::read_input;
use std::time;

fn main() {
    println!("{}", problem_a("data/day07.txt"));
    let now = time::Instant::now();
    println!("{}", problem_b("data/day07.txt"));

    let now2 = time::Instant::now();
    let ints = read_input::read_line_to_int_vec("data/day07.txt");
    println! {"Bisection: {}", bisection(1,*ints.iter().max().unwrap(),&ints) }
    println!(
        "First method took  {} microseconds",
        now.elapsed().as_micros()
    );
    println!(
        "Bisection method took  {} microseconds",
        now2.elapsed().as_micros()
    );
}

//Mean is optimal height
fn problem_a(file_name: &str) -> i64 {
    let mut depths = read_input::read_line_to_int_vec(file_name);
    depths.sort(); //easiest way (for me) to find mean...
    let mean = depths[depths.len() / 2];
    depths.iter().fold(0, |acc, el| acc + (el - mean).abs())
}

fn problem_b(file_name: &str) -> i64 {
    let depths = read_input::read_line_to_int_vec(file_name);
    let mut prev_cost = std::i64::MAX;
    //Could loop 1.. , but this will give the optimal solution, and no risk of infinite loops
    for depth in 1..depths.iter().max().unwrap() - 1 {
        let cost = depths.iter().fold(0, |acc, el| acc + cost_crab(*el, depth));
        // Check if previous cost was local minima (then i THINK it should be global minima)
        if cost > prev_cost {
            return prev_cost;
        } else {
            prev_cost = cost;
        }
    }
    panic!("no solution found") //Solution should be found by now.
}

fn bisection(left: i64, right: i64, depths: &Vec<i64>) -> i64 {
    let middle = (left + right) / 2;
    let cost_prev = depths
        .iter()
        .fold(0, |acc, el| acc + cost_crab(*el, middle - 1));
    let cost = depths
        .iter()
        .fold(0, |acc, el| acc + cost_crab(*el, middle));
    let cost_next = depths
        .iter()
        .fold(0, |acc, el| acc + cost_crab(*el, middle + 1));
    if cost > cost_prev {
        return bisection(left, middle, depths);
    } else if cost > cost_next {
        return bisection(middle, right, depths);
    } else {
        return cost;
    }
}

fn cost_crab(val: i64, height: i64) -> i64 {
    let diff = (val - height).abs();
    (diff * (diff + 1)) / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(37, problem_a("data/day07_test.txt"))
    }

    #[test]
    fn part2() {
        assert_eq!(168, problem_b("data/day07_test.txt"))
    }
}
