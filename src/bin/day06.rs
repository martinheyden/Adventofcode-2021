use aoc_2021::read_input;

const NBR_STATES: usize = 9;



fn main() {
    println!("{}", problem_a("data/day06.txt"));
    println!("{}", problem_b("data/day06.txt"));
}

fn run_n_iters(file_name: &str, nbr_iter: i64) -> i64 {
    let mut fishes = [0; NBR_STATES];
    let str = read_input::read_file_to_string(file_name);
    let nmbr_vec = str
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    for i in nmbr_vec {
        fishes[i] = fishes[i] + 1;
    }
    let mut zero_index = 0;
    for _ in 1..=nbr_iter {
        fishes[(zero_index+7)%NBR_STATES] += fishes[zero_index]; //Should update +6 of the new zero index, which is +7 of the old.
        zero_index = (zero_index + 1)%NBR_STATES;
    }
    //Count the total number of fishes
    fishes.iter().fold(0, |acc, val| acc + val)
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
