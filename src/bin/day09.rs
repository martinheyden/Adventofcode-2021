use aoc_2021::read_input;

fn main() {
    println!("{}", problem_a("data/day09.txt"));
    println!("{}", problem_b("data/day09.txt"));
}

type Matrix = Vec<Vec<i64>>;
type Cord = (usize, usize);

//Many ifs, but easy way to avoid type conversion between usize and isize due to using -
fn get_neighbours(x: usize, y: usize, x_size: usize, y_size: usize) -> Vec<Cord> {
    let mut neigh = Vec::new();
    if x > 0 {
        neigh.push((x - 1, y));
    }
    if x < x_size - 1 {
        neigh.push((x + 1, y));
    }
    if y > 0 {
        neigh.push((x, y - 1));
    }
    if y < y_size - 1 {
        neigh.push((x, y + 1))
    }
    neigh
}

fn find_basins(data: &Matrix) -> Vec<Cord> {
    let mut basins = Vec::new();
    let y_len = data.len();
    let x_len = data[0].len();
    for y in 0..y_len {
        for x in 0..x_len {
            let val = data[y][x];
            let mut low = true;
            for (xi, yi) in get_neighbours(x, y, x_len, y_len) {
                if val >= data[yi][xi] {
                    low = false;
                    break;
                }
            }
            if low {
                basins.push((x, y));
            }
        }
    }
    basins
}

fn problem_a(file_name: &str) -> usize {
    let data = read_input::read_file_to_matrix_compact(file_name);
    let basins = find_basins(&data);
    basins
        .iter()
        .fold(0, |acc, (x, y)| acc + 1 + (data[*y][*x] as usize))
}

fn problem_b(file_name: &str) -> usize {
    let data = read_input::read_file_to_matrix_compact(file_name);
    let x_len = data[0].len();
    let y_len = data.len();
    let basins = find_basins(&data);
    let mut basin_sizes: Vec<usize> = Vec::new();
    let mut visited: Vec<Vec<bool>> = Vec::new();
    for _ in 0..y_len {
        let line = vec![false; x_len];
        visited.push(line);
    }
    for (x, y) in basins.iter() {
        basin_sizes.push(1 + find_basin_size(*x, *y, &data, x_len, y_len, &mut visited));
    }
    basin_sizes.sort(); //A bit inefficient, but OK
    basin_sizes.reverse();
    basin_sizes[0] * basin_sizes[1] * basin_sizes[2]
}

fn find_basin_size(
    x: usize,
    y: usize,
    data: &Matrix,
    x_len: usize,
    y_len: usize,
    visited: &mut Vec<Vec<bool>>,
) -> usize {
    let val = data[y][x];
    let mut count = 0;
    for (xi, yi) in get_neighbours(x, y, x_len, y_len) {
        if val < data[yi][xi] && data[yi][xi] != 9 && !visited[yi][xi] {
            visited[yi][xi] = true;
            count += 1 + find_basin_size(xi, yi, data, x_len, y_len, visited);
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(15, problem_a("data/day09_test.txt"))
    }

    #[test]
    fn part2() {
        assert_eq!(1134, problem_b("data/day09_test.txt"))
    }
}
