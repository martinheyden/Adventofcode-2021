use aoc_2021::read_input;



fn main() {
    println!("{}", problem_a("data/day08.txt"));
    println!("{}", problem_b("data/day08.txt"));
}


fn problem_a(file_name: &str) -> i64 {
    let str_vec = read_input::read_file_to_string_vec(file_name);
    let mut count = 0;
    let len_vec: [usize; 4] = [2, 3, 4, 7];
    for str in str_vec {
        let mut output = false;
        for seg in str.split_whitespace() {
            if seg == "|" {
                output = true;
            }
            if output && len_vec.contains(&seg.len()) {
                count += 1;
            }
        }
    }
    count
}

fn problem_b(file_name: &str) -> usize {
    let str_vec = read_input::read_file_to_string_vec(file_name);
    let mut count = 0;
    for str in str_vec {
        let mut local_count = 0;
        let mut output = false;
        let mut mask_vec = vec!["";10];
        //First parse, find unique
        for seg in str.split_whitespace() {
            if seg == "|" {
                break;
            }
            match seg.len() {
                2 => mask_vec[1] = seg,
                3 => mask_vec[7] = seg,
                4 => mask_vec[4] = seg,
                7 => mask_vec[8] = seg,
                _ => (),
            };
        }
        //Second parse, find rest and calcualte score
        for seg in str.split_whitespace() {
            if output == true {
                local_count = local_count*10 + get_number(seg,&mask_vec);
            } else {
                if seg == "|" {
                    output = true;
                } else {
                    match seg.len() {
                        5 => {
                            if count_matches(seg,mask_vec[1]) == 2 { 
                                mask_vec[3] = seg;
                            } else if count_matches(seg, mask_vec[4]) == 2 {
                                mask_vec[2] = seg;
                            } else {
                                mask_vec[5] = seg;
                            }
                        },
                        6 => {
                            if count_matches(seg,mask_vec[1]) == 1 {
                                mask_vec[6] = seg;
                            } else if count_matches(seg,mask_vec[4]) == 4 {
                                mask_vec[9] = seg;
                            } else {
                                mask_vec[0] = seg;
                            }
                        },
                        _ => (),
                    };
                }
            }
        }
        count +=local_count;
    }
    count
}

fn count_matches(s1: &str, s2: &str) ->usize {
    let mut count = 0;
    for c1 in s1.chars() {
        for c2 in s2.chars() {
            if c1 == c2 {
                count+=1;
            }
        }
    }
    count
}

fn get_number(s:&str,v:&Vec<&str>) ->usize {
    
    for (i,s2) in v.into_iter().enumerate() {
        if contains_same(s,s2){
            return i
        }
    }
    panic!("could find number {} in {:?}", s, v);
}

fn contains_same(s1:&str,s2:&str) ->bool {
    if s1.len()!= s2.len() {
        return false
    }
    for c1 in s1.chars() {
        let mut match_found =false;
        for c2 in s2.chars(){
            if c1==c2 {
                match_found = true;
            }
        }
        if !match_found {
            return false;
        }
    }
    return true
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(26, problem_a("data/day08_test.txt"))
    }

    #[test]
    fn part2() {
        assert_eq!(61229, problem_b("data/day08_test.txt"))
    }
}
