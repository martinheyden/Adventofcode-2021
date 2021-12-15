use aoc_2021::read_input;
use std::collections::LinkedList;



fn calc_distance(cost_mat: &Vec<Vec<i64>>) -> i64 {
    let mut queue = LinkedList::new();
    let size = cost_mat.len();
    let mut path_mat = Vec::new();
    for _ in 0..size {
        path_mat.push(vec![std::i64::MAX-20;size]);
    }
    path_mat[size-1][size-1] = 0;
    queue.push_back((size-1,size-2));
    queue.push_back((size-2,size-1));
    while let Some((x,y)) = queue.pop_front(){
        if !(x==size-1 && y==size-1) {
            let mut min_val = std::i64::MAX-20;
            //Check if new path that is better
            if x>0 {
                if  path_mat[y][x] > path_mat[y][x-1] + cost_mat[y][x-1] && path_mat[y][x-1] + cost_mat[y][x-1]< min_val {
                    min_val = path_mat[y][x-1] + cost_mat[y][x-1]
                }
            }
            if y> 0 {
                if  path_mat[y][x] > path_mat[y-1][x] + cost_mat[y-1][x] && path_mat[y-1][x] + cost_mat[y-1][x]< min_val  {
                    min_val = path_mat[y-1][x] + cost_mat[y-1][x];
                }
            }
            if y < size-1 {
                if  path_mat[y][x] > path_mat[y+1][x] + cost_mat[y+1][x] && path_mat[y+1][x] + cost_mat[y+1][x]< min_val  {
                    min_val = path_mat[y+1][x] + cost_mat[y+1][x];
                }
            }
            if x < size - 1{
                if  path_mat[y][x] > path_mat[y][x+1] + cost_mat[y][x+1] && path_mat[y][x+1] + cost_mat[y][x+1]< min_val {
                    min_val = path_mat[y][x+1] + cost_mat[y][x+1];
                }
            }
            if min_val <path_mat[y][x] {
                path_mat[y][x] = min_val;
                if x>0 && min_val + cost_mat[y][x] < path_mat[y][x-1] {
                    queue.push_back((x-1,y));
                }
                if y> 0 && min_val+ cost_mat[y][x] < path_mat[y-1][x] {
                    queue.push_back((x,y-1));
                }
                if y < size-1 && min_val+ cost_mat[y][x] < path_mat[y+1][x]{
                    queue.push_back((x,y+1));
                }
                if x < size - 1 && min_val+ cost_mat[y][x] < path_mat[y][x+1]{
                    queue.push_back((x+1,y));
                }

            }
        }
    } 
    path_mat[0][0]
}

fn problem_a(file_name: &str) -> i64 {
    let cost_mat  = read_input::read_file_to_matrix_compact(file_name);
    calc_distance(&cost_mat)

}

fn problem_b(file_name: &str) -> i64 {
    let cost_mat  = read_input::read_file_to_matrix_compact(file_name);
    let cost_mat_big = create_full_mat(&cost_mat);
    calc_distance(&cost_mat_big)
}



fn create_full_mat(mat: &Vec<Vec<i64>>) ->Vec<Vec<i64>> {
    let old_size = mat.len();
    let new_size = old_size * 5;
    let mut new_mat = Vec::new();
    for _ in 0..new_size {
        new_mat.push(vec![0;new_size]);
    } 
    for i in 0..5 {
        for j in 0..5{
            for x in 0..old_size {
                for y in 0..old_size{
                    new_mat[j*old_size +y ][i*old_size+x] = (mat[y][x]-1+ (i+j) as i64)%9+1;
                }
            }
            
        }
    }
    new_mat
}

fn main() {
    println!("{}", problem_a("data/day15.txt"));
    println!("{}", problem_b("data/day15.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(40, problem_a("data/day15_test.txt"))
    }

    #[test]
    fn part2() {
        assert_eq!(315, problem_b("data/day15_test.txt"))
    }
}
