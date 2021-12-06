use aoc_2021::read_input;

const NBR_STATES: usize = 9;

fn main() {
    println!("{}", problem_a("data/day06.txt"));
    println!("{}", problem_b("data/day06.txt"));
}

fn run_n_iters(file_name: &str, nbr_iter: i64) -> i64 {
    let mut old = [0; NBR_STATES];
    let mut new = [0; NBR_STATES];
    let str = read_input::read_file_to_string(file_name);
    let nmbr_vec = str
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    //Initialize
    for i in nmbr_vec {
        old[i] = old[i] + 1;
    }
    for _day in 1..=nbr_iter {
        for i in 1..=8 {
            new[i - 1] = old[i];
        }
        new[6] = new[6] + old[0]; //Births
        new[8] = old[0];
        for i in 0..NBR_STATES {
            old[i] = new[i];
        }
    }
    //Count the total number of fishes
    old.iter().fold(0, |acc, val| acc + val)
}

fn problem_a(file_name: &str) -> i64 {
    run_n_iters(file_name, 80)
}

fn problem_b(file_name: &str) -> i64 {
    run_n_iters(file_name, 256)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(5934, problem_a("data/day06_test.txt"))
    }

    #[test]
    fn part2() {
        assert_eq!(26984457539, problem_b("data/day06_test.txt"))
    }
}
