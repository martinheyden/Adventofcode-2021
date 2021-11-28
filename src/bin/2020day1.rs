#[path = "../utils.rs"]
mod utils;


fn main() {
    println!("{}",problem_a("data/2020day1.txt"));
    println!("{}",problem_b("data/2020day1.txt"));
}

fn problem_a(file_name: &str) -> i64 {
    let data = utils::read_file_to_int_array(file_name);
    match prod_sum(data,2020) {
        Some(x) => x,
        None => panic!("..."),
    }
}

fn problem_b(file_name: &str) -> i64 {
    let data = utils::read_file_to_int_array(file_name);
    for (i,val) in data.iter().enumerate() {
        match prod_sum(data[..i].iter().chain(data[i+1..].iter()).map(|v| *v).collect(),2020-val) { 
            Some(x) => return x*val,
            None => (),
        }
    }
    return -1
}


fn prod_sum(nbrs: Vec<i64>,sum: i64) -> Option<i64> {
    for i in 0..nbrs.len() {
        for j in i+1..nbrs.len(){
            if nbrs[i] + nbrs[j] == sum {
                return Some(nbrs[i]*nbrs[j]);
            }
        }
    }
    return None;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() { 
        assert_eq!(514579,problem_a("data/2020day1_test.txt"))
    }

    #[test]
    fn part2() { 
        assert_eq!(241861950,problem_b("data/2020day1_test.txt"))
    }

}