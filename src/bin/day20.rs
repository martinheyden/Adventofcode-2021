use hashbrown::HashMap;
use itertools::Itertools;
use aoc_2021::read_input;

type Point = (isize,isize);

#[derive(Debug)]
struct DataPoint {
    lit: bool,
    filter_val: u16,
}

fn parse_input(strs: &Vec<String>) -> (Vec<bool>, HashMap<Point,DataPoint>) {
    let filter_vec = strs[0].chars().map(|x| x == '#').collect();
    let mut data_map = HashMap::new();
    for y in 2..strs.len(){
        let mut x:isize = 0;
        for ch in strs[y].chars() {
            data_map.insert((x ,(strs.len()-1-y) as isize), 
                DataPoint{lit:ch =='#', filter_val: 0 });
            x = x+1;
        } 
    }
    (filter_vec,data_map)
}

//Assumes first value is # and last value is . in filter_vec
fn update_map(filter_vec:&Vec<bool>, data_map:&mut HashMap<Point,DataPoint>,iter:usize) {
    let mut inf_val = 511; //if all lit
    if iter%2 == 0 {
        inf_val = 0; //assume all neightbours not lit
    }
    for val in data_map.values_mut() {
        val.filter_val = inf_val;
    }
    let keys = data_map.keys().map(|k| (k.0,k.1)).collect::<Vec<Point>>();
    for key in keys {
        let lit = data_map.get(&key).unwrap().lit;
        if (lit && inf_val==0) || (!lit && inf_val!= 0) {
            for (dx, dy) in (-1..=1).cartesian_product(-1..=1) {
                let f = &mut data_map.entry((key.0+dx,key.1+dy)).
                    or_insert(DataPoint{lit:false,filter_val:inf_val}).filter_val;
                if lit && inf_val == 0{
                    *f = *f + get_bit((dx,dy));
                } else if !lit && inf_val != 0{
                    *f = *f -get_bit((dx,dy));
                }
            }
        }
    }
    for val in data_map.values_mut() {
        val.lit = filter_vec[val.filter_val as usize];
    }
}

fn get_bit(v:(isize,isize)) ->u16 { 
    match v {
        (1,-1) => 256,
        (0,-1) => 128,
        (-1,-1) => 64,
        (1,0) =>32,
        (0,0) =>16,
        (-1,0) =>8,
        (1,1) =>4,
        (0,1) =>2,
        (-1,1) =>1,
        _ => panic!("unexepected tuple"),
    }
}

fn do_n_iterations(filter_vec:&Vec<bool>, data_map:&mut HashMap<Point,DataPoint>,n:usize) {
    for i in 0..n {
        update_map(filter_vec, data_map,i);
    }
}


fn main() {
    let strs = read_input::read_file_to_string_vec("data/day20.txt");
    let (filter_vec,mut data_map) = parse_input(&strs);
    do_n_iterations(&filter_vec, &mut data_map, 2);
    println!("Part a count: {}, should be 5619", data_map.iter().fold(0,|acc,entry| {if entry.1.lit {acc+1} else {acc}}));
    let strs = read_input::read_file_to_string_vec("data/day20.txt");
    let (filter_vec,mut data_map) = parse_input(&strs);
    do_n_iterations(&filter_vec, &mut data_map, 50);
    println!("Part a count: {}, should be 20122", data_map.iter().fold(0,|acc,entry| {if entry.1.lit {acc+1} else {acc}}));
}