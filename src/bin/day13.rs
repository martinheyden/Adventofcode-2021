use aoc_2021::read_input;
use regex::Regex;
use std::collections::HashSet;

//Returns a Hashset containing the dots, and a vec cotaining tuples, where the first element is true if folding in x direction, and second is where to fold
fn read_file(file_name: &str) -> (HashSet<(usize, usize)>, Vec<(bool, usize)>) {
    let mut dots = HashSet::new();
    let mut folds = Vec::new();
    let mut read_folds = false;
    let re_dot = Regex::new(r"^(\d+),(\d+)$").unwrap(); 
    let re_fold = Regex::new(r"^fold along (\w)=(\d+)$").unwrap();

    let strings = read_input::read_file_to_string_vec(file_name);
    for line in strings {
        if line == "" {
            read_folds = true;
        } else if !read_folds {
            let c = re_dot.captures(&line).unwrap();
            dots.insert((
                c[1].parse::<usize>().unwrap(),
                c[2].parse::<usize>().unwrap(),
            ));
        } else {
            let c = re_fold.captures(&line).unwrap();
            folds.push((c[1].eq("x"), c[2].parse::<usize>().unwrap()));
        }
    }
    (dots, folds)
}

fn fold(
    dots: &HashSet<(usize, usize)>,
    fold_along_x: bool,
    fold_at: usize,
) -> HashSet<(usize, usize)> {
    let mut new_dots = HashSet::new();
    for dot in dots.iter() {
        match mirror_dot(*dot, fold_along_x, fold_at) {
            Some(d) => new_dots.insert(d),
            None => false, //annoying
        };
    }
    new_dots
}

fn mirror_dot(dot: (usize, usize), fold_along_x: bool, fold_at: usize) -> Option<(usize, usize)> {
    if fold_along_x && dot.0 != fold_at {
        if dot.0 < fold_at {
            Some((dot.0, dot.1))
        } else {
            Some((dot.0 - (dot.0 - fold_at) * 2, dot.1))
        }
    } else if !fold_along_x && dot.1 != fold_at {
        if dot.1 < fold_at {
            Some((dot.0, dot.1))
        } else {
            Some((dot.0, dot.1 - (dot.1 - fold_at) * 2))
        }
    } else {
        None
    }
}

fn visualize_dots(dots: HashSet<(usize, usize)>, xsize: usize, ysize: usize) {
    let mut mat = Vec::new();
    for _ in 0..ysize {
        let vec = vec!['💚'; xsize];
        mat.push(vec);
    }
    for dot in dots {
        mat[dot.1][dot.0] = '💗'; //not recomended for red-green colorblind...
    }
    for i in 0..ysize {
        println!("{}",
            mat[i].iter().fold(String::new(), |mut acc, el| {
                acc.push(*el);
                acc
            })
        ); //Map vector to string
    }
}
//
fn problem_a(file_name: &str) -> usize {
    let (dots, folds) = read_file(file_name);
    let dots = fold(&dots, folds[0].0, folds[0].1);
    dots.len()
}

fn problem_b(file_name: &str) {
    let (mut dots, folds) = read_file(file_name);
    let mut xsize = 0;
    let mut ysize = 0;
    for f in folds {
        if f.0 {
            xsize = f.1;
        } else {
            ysize = f.1;
        }
        dots = fold(&dots, f.0, f.1);
    }
    visualize_dots(dots, xsize, ysize);
}

fn main() {
    println!("{}", problem_a("data/day13.txt"));
    problem_b("data/day13.txt")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(17, problem_a("data/day13_test.txt"))
    }
}
