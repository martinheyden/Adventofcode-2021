use aoc_2021::read_input;

//Each element is either a new par, or a single number
enum Element {
    Pair(Snailnumber),
    Single(usize),
}

//Each pair as a left and a right element
struct Snailnumber {
    left: Box<Element>,
    right: Box<Element>,
}

fn read_snailnumber(line: &mut Vec<char>) -> Element {
    let ch = line.remove(0);
    match ch {
        '[' => {
            let left = read_snailnumber(line);
            line.remove(0); // read comma
            let right = read_snailnumber(line);
            line.remove(0); // read ]
            let this = Element::Pair(Snailnumber {
                left: Box::new(left),
                right: Box::new(right),
            });

            return this;
        }
        _ => return Element::Single(ch.to_digit(10).unwrap() as usize),
    }
}

fn explode(
    ele: &mut Element,
    last_left: Option<&mut Element>,
    last_right: Option<&mut Element>,
    depth: usize,
    exploded: &mut bool,
) -> bool {
    if depth == 4 {
        match ele {
            Element::Single(_) => (),
            Element::Pair(p) => {
                match *p.left {
                    Element::Single(s) => {
                        match last_left {
                            Some(left_el) => add_right(left_el, s),
                            _ => (),
                        };
                    }
                    Element::Pair(_) => panic!("did not expect pair"),
                };
                match *p.right {
                    Element::Single(s) => {
                        match last_right {
                            Some(right_el) => add_left(right_el, s),
                            _ => (),
                        };
                    }
                    Element::Pair(_) => panic!("did not expect pair"),
                };
                *exploded = true;
                return true;
            }
        }
    } else {
        match ele {
            Element::Single(_) => (),
            Element::Pair(sn) => {
                if !*exploded
                    && explode(
                        &mut *sn.left,
                        last_left,
                        Some(&mut *sn.right),
                        depth + 1,
                        exploded,
                    )
                {
                    sn.left = Box::new(Element::Single(0));
                }

                if !*exploded
                    && explode(
                        &mut *sn.right,
                        Some(&mut *sn.left),
                        last_right,
                        depth + 1,
                        exploded,
                    )
                {
                    sn.right = Box::new(Element::Single(0));
                }
            }
        }
    }
    return false;
}

fn add_left(ele: &mut Element, val: usize) {
    match ele {
        Element::Pair(p) => add_left(&mut *p.left, val),
        Element::Single(s) => *s += val,
    };
}

fn add_right(ele: &mut Element, val: usize) {
    match ele {
        Element::Pair(p) => add_right(&mut *p.right, val),
        Element::Single(s) => *s += val,
    };
}

fn split(ele: &mut Element, splitted: &mut bool) -> usize {
    match ele {
        Element::Pair(p) => {
            if !*splitted {
                split_arm(&mut p.left,splitted);
            }
            if !*splitted {
                split_arm(&mut p.right,splitted);
            }
        }
        Element::Single(s) => {
            if *s > 9 {
                *splitted = true;
                return *s;
            };
        }
    }
    return 1;
}

fn split_arm(ele: &mut Box<Element>, splitted: &mut bool) {
    let val = split(ele, splitted);
    if val > 9 {
        *ele = Box::new(create_split_pair(val));
    }
}

fn create_split_pair(val: usize) -> Element {
    let left = val / 2;
    let right = val / 2 + val % 2;
    Element::Pair(Snailnumber {
        left: Box::new(Element::Single(left)),
        right: Box::new(Element::Single(right)),
    })
}

fn reduce(el: &mut Element) {
    let mut exploded = true;
    let mut splitted = true;
    while splitted {
        if exploded {
            exploded = false;
            explode(el, None, None, 0, &mut exploded);
            splitted = true;
        } else {
            splitted = false;
            split(el, &mut splitted);
            if splitted {
                exploded = true;
            }
        }
    }
}

fn add(e1: Element, e2: Element) -> Element {
    Element::Pair(Snailnumber {
        left: Box::new(e1),
        right: Box::new(e2),
    })
}

fn string_to_char_vec(s: &str) -> Vec<char> {
    s.chars().collect()
}

fn calc_magnitue(el: &Element) -> usize {
    match el {
        Element::Single(s) => return *s,
        Element::Pair(p) => return 3 * calc_magnitue(&p.left) + 2 * calc_magnitue(&p.right),
    }
}

fn problem_a(file_name: &str) -> usize {
    let str_vec = read_input::read_file_to_string_vec(file_name);
    let mut init = false;
    let mut el = read_snailnumber(&mut string_to_char_vec(&str_vec[0])); // Todo...
    for s in str_vec {
        let new_el = read_snailnumber(&mut string_to_char_vec(&s));
        if !init {
            el = new_el;
            init = true;
        } else {
            el = add(el, new_el);
        }
        reduce(&mut el);
    }
    calc_magnitue(&el)
}

fn problem_b(file_name: &str) -> usize {
    let mut best = 0;
    let str_vec = read_input::read_file_to_string_vec(file_name);
    for i in 0..str_vec.len() {
        for j in 0..str_vec.len() {
            let e1 = read_snailnumber(&mut string_to_char_vec(&str_vec[i]));
            let e2 = read_snailnumber(&mut string_to_char_vec(&str_vec[j]));
            let mut el = add(e1, e2);
            reduce(&mut el);
            let mag = calc_magnitue(&el);
            if mag > best {
                best = mag;
            }
        }
    }
    best
}

fn main() {
    println!("{}", problem_a("data/day18.txt"));
    println!("{}", problem_b("data/day18.txt"));
}

//Some functions useful for debugging
fn to_char(n: usize) -> char {
    match n {
        0 => '0',
        1 => '1',
        2 => '2',
        3 => '3',
        4 => '4',
        5 => '5',
        6 => '6',
        7 => '7',
        8 => '8',
        9 => '9',
        10 => 'A',
        11 => 'B',
        12 => 'C',
        13 => 'D',
        14 => 'E',
        15 => 'F',
        16 => 'G',
        17 => 'H',
        18 => 'I',
        19 => 'J',
        _ => 'X',
    }
}

impl Element {
    fn to_string(&self, s: &mut String) {
        match self {
            Element::Pair(sn) => sn.to_string(s),
            Element::Single(n) => s.push(to_char(*n)),
        };
    }
}

impl Snailnumber {
    fn to_string(&self, s: &mut String) {
        s.push('[');
        self.left.to_string(s);
        s.push(',');
        self.right.to_string(s);
        s.push(']');
    }
}
