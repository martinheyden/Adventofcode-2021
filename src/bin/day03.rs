
use aoc_2021::read_input as read_input;

fn main() {
    println!("{}",problem_a("data/day03.txt"));
    println!("{}",problem_b("data/day03.txt"));
}



fn problem_a(filename: &str) ->i64 {
    let lines = read_input::read_file_to_string_vec(filename);
    let nbr_of_bits = lines[0].chars().count();
    let mut count = Vec::with_capacity(nbr_of_bits);
    count.resize(nbr_of_bits, 0);
    for line in  lines.iter() {
        for (nbr,ch) in line.chars().enumerate() {
            match ch {
                '0' => count[nbr] = count[nbr] - 1,
                '1' => count[nbr] = count[nbr] + 1,
                _ => panic!("input should be zeros and ones"),
            }
        }
    }

    find_params(&count)
}



fn find_params(bits: &Vec<i64>) -> i64 {
    let mut gamma = 0;
    let mut epsilon = 0;
    let mut base = 1;
    for i in bits.iter().rev() {
        if *i > 0 { 
            gamma+= base;
        }else if *i < 0{
            epsilon += base;
        } else {
            panic!("equal number of bits");
        }
        base *= 2;
    }
    gamma*epsilon
}


fn problem_b(filename: &str) ->i64 {
    let lines = read_input::read_file_to_string_vec(filename); // Vec<String> so that we have ownership
    let oxygen = calc_parameter(&lines,|x| x>=0);
    let co2 = calc_parameter(&lines,|x| x <0);
    co2*oxygen
}

//one_logic describes if a number will be saved if the current bit is one for a given count.
fn calc_parameter(data: &Vec<String>, one_logic: fn(i64) -> bool ) -> i64 { 
    let mut vec = data.iter().collect::<Vec<&String>>(); //Makes it Vec<&String>
    // Must be a better way to do the above
    let nbr_of_bits = vec[0].chars().count();

    for i in 0..nbr_of_bits {
        let count = vec.iter().fold(0,|count,line| 
            match line.chars().nth(i).unwrap() {
                '1' => count+1,
                '0' => count-1,
                _ => panic!(""),
            });
        if vec.iter().count()>1 { //if more than one number left
            vec = vec.into_iter().filter(|line| { //into iter,  otherwise we get &&string
                match line.chars().nth(i).unwrap() {
                    '1' => one_logic(count),
                    '0' => !one_logic(count),
                    _ => panic!(""),
                }
            }).collect();
        }
    }
    i64::from_str_radix(vec[0],2).unwrap()
}




#[cfg(test)]
mod tests_day3 {
    use super::*;

    #[test]
    fn part1() { 
        assert_eq!(198,problem_a("data/day03_test.txt"))
    }

    #[test]
    fn part2() { 
        assert_eq!(230,problem_b("data/day03_test.txt"))
    }

}