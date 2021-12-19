use aoc_2021::read_input;
use regex::Regex;
use std::collections::HashSet;

type SateliteMeasurement = Vec<Coordinate>;

#[derive(Debug)]
enum Axis {
    X(isize),
    Y(isize),
    Z(isize),
}

#[derive(Debug)]
struct BasisChange{
    x: Axis,
    y: Axis,
    z: Axis,
}

#[derive(Debug)]
#[derive(Hash)]
struct Coordinate {
    x: isize,
    y: isize,
    z: isize,
}

impl Coordinate {
    fn new(x:isize,y:isize,z:isize) -> Coordinate {
        Coordinate{ x:x, y:y, z:z}
    }

    fn change_base(&self,base: &BasisChange) ->Coordinate {
        let mut c = Coordinate{x:0,y:0,z:0};
        match base.x {
            Axis::X(v) => c.x = self.x*v,
            Axis::Y(v) => c.y = self.x*v,
            Axis::Z(v) => c.z = self.x*v,
        }
        match base.y {
            Axis::X(v) => c.x = self.y*v,
            Axis::Y(v) => c.y = self.y*v,
            Axis::Z(v) => c.z = self.y*v,
        }
        match base.z {
            Axis::X(v) => c.x = self.z*v,
            Axis::Y(v) => c.y = self.z*v,
            Axis::Z(v) => c.z = self.z*v,
        }
        c
    }

    fn displace(&self, vec:&Coordinate) -> Coordinate {
        Coordinate{x: vec.x+self.x, y: vec.y+self.y, z: vec.z+self.z}
    }

    fn displace_negative(&self, vec:&Coordinate) -> Coordinate {
        Coordinate{x: -vec.x+self.x, y: -vec.y+self.y, z: -vec.z+self.z}
    }

    fn change_and_move(&self,base: &BasisChange, vec:&Coordinate) -> Coordinate {
        self.change_base(base).displace(vec)
    }

    fn translation_to(&self, other: &Coordinate) ->Coordinate{
        Coordinate{x: other.x-self.x, y: other.y-self.y, z: other.z-self.z}
    }

    fn in_range(&self) -> bool {
        self.x <1000 && self.y <1000 && self.z <1000
    }

    fn equals(&self, other: &Coordinate) ->bool {
        self.x==other.x && self.y ==other.y && self.z == other.z
    }

    fn distance(&self, other: &Coordinate) ->usize {
        (num::abs(self.x-other.x) + num::abs(self.y-other.y) + num::abs(self.z-other.z)) as usize
    }
}

impl PartialEq for Coordinate {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y &&self.z == other.z 
    }
}
impl Eq for Coordinate {}

impl BasisChange{
    fn new_vec() -> Vec<BasisChange> {
        let mut v = Vec::new();
        v.push(BasisChange{x: Axis::X(1), y: Axis::Y(1), z: Axis::Z(1)});
        v.push(BasisChange{x: Axis::X(1), y: Axis::Y(-1), z: Axis::Z(-1)});
        v.push(BasisChange{x: Axis::X(-1), y: Axis::Y(1), z: Axis::Z(-1)});
        v.push(BasisChange{x: Axis::X(-1), y: Axis::Y(-1), z: Axis::Z(1)});
        //
        v.push(BasisChange{x: Axis::Y(1), y: Axis::X(1), z: Axis::Z(-1)});
        v.push(BasisChange{x: Axis::Y(1), y: Axis::X(-1), z: Axis::Z(1)});
        v.push(BasisChange{x: Axis::Y(-1), y: Axis::X(1), z: Axis::Z(1)});
        v.push(BasisChange{x: Axis::Y(-1), y: Axis::X(-1), z: Axis::Z(-1)});
        //
        v.push(BasisChange{x: Axis::X(1), y: Axis::Z(1), z: Axis::Y(-1)});
        v.push(BasisChange{x: Axis::X(1), y: Axis::Z(-1), z: Axis::Y(1)});
        v.push(BasisChange{x: Axis::X(-1), y: Axis::Z(1), z: Axis::Y(1)});
        v.push(BasisChange{x: Axis::X(-1), y: Axis::Z(-1), z: Axis::Y(-1)});
        //
        v.push(BasisChange{x: Axis::Z(1), y: Axis::X(1), z: Axis::Y(1)});
        v.push(BasisChange{x: Axis::Z(1), y: Axis::X(-1), z: Axis::Y(-1)});
        v.push(BasisChange{x: Axis::Z(-1), y: Axis::X(1), z: Axis::Y(-1)});
        v.push(BasisChange{x: Axis::Z(-1), y: Axis::X(-1), z: Axis::Y(1)});
        //
        v.push(BasisChange{x: Axis::Y(1), y: Axis::Z(1), z: Axis::X(1)});
        v.push(BasisChange{x: Axis::Y(1), y: Axis::Z(-1), z: Axis::X(-1)});
        v.push(BasisChange{x: Axis::Y(-1), y: Axis::Z(1), z: Axis::X(-1)});
        v.push(BasisChange{x: Axis::Y(-1), y: Axis::Z(-1), z: Axis::X(1)});
        //
        v.push(BasisChange{x: Axis::Z(1), y: Axis::Y(1), z: Axis::X(-1)});
        v.push(BasisChange{x: Axis::Z(1), y: Axis::Y(-1), z: Axis::X(1)});
        v.push(BasisChange{x: Axis::Z(-1), y: Axis::Y(1), z: Axis::X(1)});
        v.push(BasisChange{x: Axis::Z(-1), y: Axis::Y(-1), z: Axis::X(-1)});
        v
    }


}

fn check_overlap(s1: &SateliteMeasurement, 
    s2: &SateliteMeasurement,
    bcs: &Vec<BasisChange>) -> Option<(Coordinate,usize)> {
        //Find candidate translation and basis change
        for p1 in s1 {
            for p2 in s2 {
                for i in  0..bcs.len() {
                    let bc = &bcs[i];
                    //Fint the location of the coordinate frame
                    let translation = p1.displace_negative(&p2.change_base(bc));
                    let count = count_matches(s1,s2,bc,&translation);
                    if count >=12 {
                        return Some((translation,i))
                    }
                    //Count matches
                }
            }
        }
        return None
}

fn count_matches(s1:&SateliteMeasurement,
    s2:&SateliteMeasurement,
    bc:&BasisChange,
    t: &Coordinate) -> usize {
        let mut count = 0;
        for p1 in s1 {
            for p2 in s2{
                if p1.equals(&p2.change_and_move(bc,t)) {
                    count += 1;
                    if count == 12 {
                        println!("{:?}", p1)
                    }
                }
            }
        }
        count
}

fn match_and_translate(sats:&mut Vec<SateliteMeasurement>) -> 
(Vec<SateliteMeasurement>, Vec<Coordinate>) {
    let mut sats_in_0_frame = Vec::new();
    sats_in_0_frame.push(sats.remove(0));
    let mut sat_locs = Vec::new();
    let bv = BasisChange::new_vec();
    while sats.len()!=0 {
        'outer:
        for i in 0..sats.len(){
            for j in 0..sats_in_0_frame.len() {
                match check_overlap(& sats_in_0_frame[j],&sats[i], &bv) {
                    None => (),
                    Some(t) => {
                        println!("Translation: {:?}", t.0);
                        sats_in_0_frame.push(sats.remove(i).iter()
                            .map(|x| x.change_and_move(&bv[t.1],&t.0)).collect());
                        sat_locs.push(t.0);
                          //   println!{"points in reference frame {:?}", sats_in_0_frame};
                        break 'outer;
                    },
                }
            } 
        }
    }
    (sats_in_0_frame, sat_locs)
}

fn count_unique(sats: &Vec<SateliteMeasurement>) -> usize {
    let mut set = HashSet::new();
    let mut count = 0;
    for sat in sats {
        for c in sat {
            match set.insert(c) {
                true => count+= 1,
                false => (),
            }
        }
    }
    count
}
   

fn max_distance(trans: &Vec<Coordinate>) -> usize {
    let mut max_dist = 0;
    for i in 0..trans.len() {
        for j in i+1..trans.len(){
            let dist = trans[i].distance(&trans[j]);
            if dist > max_dist {
                max_dist = dist;
            }
        }
    }
    max_dist
}


fn parse_input(strs: &Vec<String>) -> Vec<SateliteMeasurement> {
    let mut v = Vec::new();
    let mut sat = Vec::new();
    let re = Regex::new(r"(-*)(\d+),(-*)(\d+),(-*)(\d+)").unwrap();
    let mut i = 1;
    while  i<strs.len() {
        if strs [i]== "" {
            v.push(sat);
            sat = Vec::new();
            i = i+2;
        } else {
            let c = re.captures(&strs[i]).unwrap();
            let mut x = c[2].parse::<isize>().unwrap();
            if c[1].len() == 1 {
                x = -x;
            }
            let mut y = c[4].parse::<isize>().unwrap();
            if c[3].len() == 1 {
                y = -y;
            }
            let mut z = c[6].parse::<isize>().unwrap();
            if c[5].len() == 1 {
                z = -z;
            }
            sat.push(Coordinate{x:x,y:y,z:z});
            i = i+1;
        }
    }
    v.push(sat);
    v
}



fn main() {
    let a = Coordinate::new(1,2,3);
    let b = Coordinate::new(2,1,-3);
    let bv = BasisChange::new_vec();
    println!("{:?}",b.change_base(&bv[4]));

    let c = Coordinate::new(1,1,0);
    let v = Coordinate::new(2,2,0);
    println!("{:?}",c.change_and_move(&bv[3], &v));

    let strs = read_input::read_file_to_string_vec(&"data/day19.txt");
    let mut data = parse_input(&strs);
    println!("{:?}",check_overlap(&data[0], &data[1], &bv));
    println!("{:?}",check_overlap(&data[1], &data[4], &bv));
    let (sats,trans) = match_and_translate(&mut data);
    println!("{}",count_unique(&sats));
    println!("{}",max_distance(&trans));
   // println!("{:?}",data);
}