use aoc_2021::read_input;
use std::collections::{HashMap, HashSet};

pub enum Status {
    Done(i64),
    NotDone,
}
//Trying out the trait system, this (not the trait system) is a bit wonky
pub trait BingoManager {
    fn new_bingo(&mut self, id: usize, board: &BingoBoard, n: i64, nbr_of_boards: usize);
    fn has_bingo(&self, id: usize) -> bool;
    fn is_done(&self) -> Status;
}

struct FirstBingo {
    score: i64,
}

impl BingoManager for FirstBingo {
    fn new_bingo(&mut self, _id: usize, board: &BingoBoard, n: i64, _nbr_of_boards: usize) {
        self.score = calc_answer(board, n);
    }

    //will end after first bingo
    fn has_bingo(&self, _id: usize) -> bool {
        false
    }

    fn is_done(&self) -> Status {
        if self.score == -1 {
            Status::NotDone
        } else {
            Status::Done(self.score)
        }
    }
}

struct LastBingo {
    score: i64,
    done: usize,
    bingos: HashSet<usize>,
}

impl BingoManager for LastBingo {
    fn new_bingo(&mut self, id: usize, board: &BingoBoard, n: i64, nbr_of_boards: usize) {
        self.bingos.insert(id);
        self.done += 1; //Increase count
                        //And check if done
        if self.done == nbr_of_boards {
            self.score = calc_answer(board, n);
        }
    }

    fn has_bingo(&self, id: usize) -> bool {
        match self.bingos.get(&id) {
            Some(_s) => true,
            None => false,
        }
    }

    fn is_done(&self) -> Status {
        if self.score == -1 {
            Status::NotDone
        } else {
            Status::Done(self.score)
        }
    }
}

const BOARD_SIZE: usize = 5;

//Keep track of the value, and if it been "marked"
pub struct BingoEntry {
    value: i64,
    marked: bool,
}

type Bingoline = Vec<BingoEntry>; // A line in a bingo square
pub type BingoBoard = Vec<Bingoline>; //One Bingo square

fn main() {
    println!("{}", problem_a("data/day04.txt"));
    println!("{}", problem_b("data/day04.txt"));
}

// Returns a tuple containing: The list of numbers to be drawn, A vector containing all the Bingo-boards
// And a map that tells in which BingoBoard(s) and in what coordinate each number is in
fn read_bingos(
    file_name: &str,
) -> (
    Vec<i64>,
    Vec<BingoBoard>,
    HashMap<i64, Vec<(usize, usize, usize)>>,
) {
    let data_vec = read_input::read_file_to_string_vec(file_name);
    let nbrs: Vec<i64> = data_vec[0] //read first line
        .split(",")
        .map(|s| s.parse::<i64>().unwrap())
        .collect();
    let mut bingos: Vec<BingoBoard> = Vec::new();
    let mut i: usize = 2; //second line is blank
    let mut bingo_count = 0;
    let mut number_map: HashMap<i64, Vec<(usize, usize, usize)>> = HashMap::new();
    while i <= data_vec.len() - BOARD_SIZE {
        let mut bingo: BingoBoard = Vec::new();
        for line in 0..BOARD_SIZE {
            let mut bingo_line: Vec<BingoEntry> = Vec::new();
            let mut column = 0;
            for nbr_str in data_vec[i + line].split_whitespace() {
                let nbr = nbr_str.parse::<i64>().unwrap();
                bingo_line.push(BingoEntry {
                    value: nbr,
                    marked: false,
                });
                let tuple_vec = &mut number_map.entry(nbr).or_insert(Vec::new());
                tuple_vec.push((bingo_count, line, column));
                column += 1;
            }
            bingo.push(bingo_line);
        }
        bingos.push(bingo);
        i = i + BOARD_SIZE + 1;
        bingo_count += 1;
    }
    (nbrs, bingos, number_map)
}

fn play_bingo(file_name: &str, manager: &mut dyn BingoManager) -> i64 {
    let (nbrs, mut bingos, number_map) = read_bingos(file_name);
    for n in nbrs {
        let tuple_vec = number_map.get(&n).unwrap();
        for e in tuple_vec {
            bingos[e.0][e.1][e.2].marked = true;
            if !manager.has_bingo(e.0) {
                if check_bingo(&bingos[e.0], e.1, e.2) {
                    manager.new_bingo(e.0, &bingos[e.0], n, bingos.len());
                    match manager.is_done() {
                        Status::Done(i) => return i,
                        Status::NotDone => (),
                    }
                }
            }
        }
    }
    return -1;
}

fn check_bingo(bingo: &BingoBoard, i: usize, j: usize) -> bool {
    let mut bingo_i = true;
    let mut bingo_j = true;
    for jj in 0..BOARD_SIZE {
        if !bingo[i][jj].marked {
            bingo_j = false
        }
    }
    for ii in 0..BOARD_SIZE {
        if !bingo[ii][j].marked {
            bingo_i = false
        }
    }
    bingo_i || bingo_j
}

fn calc_answer(bingo: &BingoBoard, n: i64) -> i64 {
    let mut sum = 0;
    for i in 0..BOARD_SIZE {
        for j in 0..BOARD_SIZE {
            if !bingo[i][j].marked {
                sum += bingo[i][j].value;
            }
        }
    }
    sum * n
}

fn problem_a(file_name: &str) -> i64 {
    let mut manager = FirstBingo { score: -1 };
    play_bingo(file_name, &mut manager)
}

fn problem_b(file_name: &str) -> i64 {
    let mut manager = LastBingo {
        score: -1,
        done: 0,
        bingos: HashSet::new(),
    };
    play_bingo(file_name, &mut manager)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(4512, problem_a("data/day04_test.txt"))
    }

    #[test]
    fn part2() {
        assert_eq!(-1, problem_b("data/day04_test.txt"))
    }
}
