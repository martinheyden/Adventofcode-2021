use aoc_2021::read_input;

fn main() {
    println!("{}", problem_a("data/day07.txt"));
    println!("{}", problem_b("data/day07.txt"));
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
