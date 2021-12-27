use aoc_2021::read_input;

#[derive(Clone,Copy,Debug)]
enum Cocumber {
    South,
    East,
    Empty,
}

type Point = (usize,usize);
type Matrix = Vec<Vec<Cocumber>>;

impl Cocumber {
    fn get_next_position(&self,x:usize,y:usize,xmax:usize, ymax:usize,east:bool) ->Option<Point> {
        match self {
            Cocumber::South => {
                if east {
                    return None;
                }
                if y> 0 {
                    return Some((x,y-1));
                } else { 
                    return Some((x,ymax)); //cant move
                }

            },
            Cocumber::East => {
                if !east {
                    return None;
                }
                if x==xmax {
                    return Some((0,y))
                } else {
                    return Some((x+1,y))
                }
            }
            Cocumber::Empty => return None,
        }
    }
}


fn read_data(file:&str) -> (Matrix,Matrix) {
    let mut current = Vec::new();
    let mut next = Vec::new();
    let lines = read_input::read_file_to_string_vec(file);
    for i in (0..lines.len()).rev() {
        let line = &lines[i];
        let mut slice = Vec::new();
        for ch in line.chars() {
            if ch == '>' {
                slice.push(Cocumber::East);
            } else if ch == 'v' {
                slice.push(Cocumber::South);
            } else {
                slice.push(Cocumber::Empty);
            }
        }
        current.push(slice);
        next.push(vec![Cocumber::Empty;line.len()]);
    }
    (current,next)
}

fn copy(next: &mut Matrix,cur: &mut Matrix) {
    for x in 0..next[0].len() {
        for y in 0..next.len() {
            next[y][x] = cur[y][x];
        }
    }
}

fn move_group(current: &mut Matrix, next: &mut Matrix, east:bool, movement: &mut bool) {
    copy(next, current);
    let xmax = current[1].len();
    let ymax = current.len();
    for x in 0..xmax {
        for y in 0..ymax {
            match current[y][x].get_next_position(x,y,xmax -1,ymax-1,east) {
                None => (),
                Some ((xn,yn)) => {
                    match  current[yn][xn] {
                        Cocumber::Empty  => {
                            *movement = true;
                            if east {
                                next[yn][xn] = Cocumber::East;
                            } else {
                                next[yn][x] = Cocumber::South;
                            }
                            next[y][x] = Cocumber::Empty;
                        },
                        _ => (),
                    };
                }
            }
        }
    }
}

fn solve (file:&str) -> usize {
    let mut iter = 0;
    let mut movement = false;
    let (mut current,mut next) = read_data(file);
    loop {
        movement = false;
        move_group(&mut current, &mut next, true, &mut movement);
        move_group(&mut next, &mut current, false, &mut movement);
        iter = iter+1;
        if !movement {
            return iter
        }
    }
}



fn main() {
    println!("{}",solve("data/day25.txt"))
}