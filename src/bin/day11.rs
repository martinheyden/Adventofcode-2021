use aoc_2021::read_input;
use itertools::Itertools;
fn main() {
    println!("{}", problem_a("data/day11.txt"));
    println!("{}", problem_b("data/day11.txt"));
}

fn one_time_step(mat: &mut Vec<Vec<i64>>, blinkers: &mut Vec<(usize, usize)>) -> usize {
    let mut count = 0;
    blinkers.clear();
    for (x, y) in (0..10).cartesian_product(0..10) {
        mat[y][x] += 1;
        if mat[y][x] == 10 {
            blinkers.push((x, y));
            count += 1;
        }
    }
    while let Some((x, y)) = blinkers.pop() {
        //The order shouldnt matter
        for (dx, dy) in (0..3).cartesian_product(0..3) {
            if x + dx > 0 && x + dx - 1 < 10 && y + dy > 0 && y + dy - 1 < 10 {
                mat[y + dy - 1][x + dx - 1] += 1;
                if mat[y + dy - 1][x + dx - 1] == 10 {
                    //Need to check exactly 10, because i let the energy go higher
                    blinkers.push((x + dx - 1, y + dy - 1));
                    count += 1;
                }
            }
        }
    }
    for (x, y) in (0..10).cartesian_product(0..10) {
        if mat[y][x] >= 10 {
            mat[y][x] = 0;
        }
    }
    count
}

fn problem_a(file_name: &str) -> usize {
    let mut mat = read_input::read_file_to_matrix_compact(file_name);
    let mut count = 0;
    let mut blinkers = Vec::new();
    for _ in 0..100 {
        count += one_time_step(&mut mat, &mut blinkers)
    }
    count
}

fn problem_b(file_name: &str) -> usize {
    let mut mat = read_input::read_file_to_matrix_compact(file_name);
    let mut blinkers = Vec::new();
    for i in 0..100000 {
        //to avoid infnite loop in case of bad programming
        if one_time_step(&mut mat, &mut blinkers) == 100 {
            return i + 1;
        }
    }
    panic!("didnt find sync flash");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(1656, problem_a("data/day11_test.txt"))
    }

    #[test]
    fn part2() {
        assert_eq!(195, problem_b("data/day11_test.txt"))
    }
}
