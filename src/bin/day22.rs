use regex::Regex;
use aoc_2021::read_input;
use itertools::Itertools;
use std::cmp::min;
use std::cmp::max;
use hashbrown::HashSet;

#[derive(Debug)]
struct Rectangle {
    xmin: isize,
    xmax: isize,
    ymin: isize,
    ymax: isize,
    zmin: isize,
    zmax: isize,
    on:bool,
}

impl Rectangle {
    fn new_vec(v: &[isize;6],on: bool) ->Rectangle {
        Rectangle{ xmin:v[0], xmax:v[1],ymin:v[2],ymax:v[3],zmin:v[4],zmax:v[5],on:on}
    }

    fn new(xmin:isize,xmax:isize,ymin:isize,ymax:isize,zmin:isize,zmax:isize) ->Rectangle {
        Rectangle{xmin:xmin, xmax:xmax,ymin:ymin,ymax:ymax,zmin:zmin,zmax:zmax,on:true}
    }

    fn intersect(&self, other: &Rectangle)->bool {
        if self.xmax < other.xmin || self.xmin > other.xmax {
            return false;
        }
        if self.ymax < other.ymin || self.ymin > other.ymax {
            return false;
        }
        if self.zmax < other.zmin || self.zmin > other.zmax {
            return false;
        }
        true
    }


    fn subtract(&self, o:&Rectangle, news: &mut Vec<Rectangle>){
        let c = Rectangle{	
            xmin:max(self.xmin, o.xmin),
            xmax:min(self.xmax, o.xmax),
            ymin:max(self.ymin, o.ymin),
            ymax:min(self.ymax, o.ymax),
            zmin:max(self.zmin, o.zmin),
            zmax:min(self.zmax, o.zmax),
            on:true};
        if self.zmin<c.zmin	{
            news.push(Rectangle::new(self.xmin, self.xmax, self.ymin, self.ymax, self.zmin, c.zmin-1)); // bottom part
        }
        if c.zmax<self.zmax {
            news.push(Rectangle::new(self.xmin, self.xmax, self.ymin, self.ymax, c.zmax+1, self.zmax)); // bottom part
        }	
        // middle portions
        if self.xmin<c.xmin {
            news.push(Rectangle::new(self.xmin, c.xmin-1, self.ymin, self.ymax, c.zmin, c.zmax)); 
        }	
        if c.xmax<self.xmax {
            news.push(Rectangle::new(c.xmax+1, self.xmax, self.ymin, self.ymax, c.zmin, c.zmax));
        }	
        if self.ymin<c.ymax {
            news.push(Rectangle::new(c.xmin, c.xmax, self.ymin, c.ymin-1, c.zmin, c.zmax));
        }	
        if c.ymax<self.ymax {
            news.push(Rectangle::new(c.xmin, c.xmax, c.ymax+1, self.ymax, c.zmin, c.zmax));
        }	
    }

    fn volume(&self) -> isize {
        (self.xmax+1-self.xmin) * (self.ymax+1-self.ymin)*(self.zmax+1-self.zmin)
    }
}


fn part2(file:&str) ->isize {
    let mut recs: Vec<Rectangle>  = Vec::new();
    let mut new_recs:  Vec<Rectangle>  = Vec::new();
    let inp = read_input::read_file_to_string_vec(file);
    for s in inp {
        let new = parse_line(&s);
        for old in recs{
            if  old.intersect(&new) {
                old.subtract(&new,&mut new_recs);
            } else { // no intersection
                new_recs.push(old);
            }
        }
        if new.on {
            new_recs.push(new);
        }
        recs = new_recs;
        new_recs = Vec::new()
    }
    recs.iter().fold(0,|acc,r| acc + r.volume())
}



fn parse_line(s :&str) -> Rectangle {
    let re = Regex::new(r"(\w+) x=(-*)(\d+)..(-*)(\d+),y=(-*)(\d+)..(-*)(\d+),z=(-*)(\d+)..(-*)(\d+)").unwrap();
    let mut data = [0;6];
    let c = re.captures(s).unwrap();
    let mut on = true;
    if "off".eq(&c[1]) {
        on = false;
    }
    for i in 0..6 {
        if c[2+i*2].len() == 1 {
            data[i] = - c[3+i*2].parse::<isize>().unwrap();
        } else {
            data[i] = c[3+i*2].parse::<isize>().unwrap();
        }
    }
    Rectangle::new_vec(&data,on)
}


fn part_a(s:&str) -> usize {
    let strs = read_input::read_file_to_string_vec(s);
    let mut set = HashSet::new();
    let mut count = 0;
    for line in strs {
        let rec = parse_line(&line);
        for ((x,y),z) in (max(rec.xmin,-50)..=min(rec.xmax,50))
            .cartesian_product(max(rec.ymin,-50)..=min(rec.ymax,50))
            .cartesian_product(max(rec.zmin,-50)..=min(rec.zmax,50)) {
            let t = (x,y,z);
            if !set.contains(&t) && rec.on {
                count +=1;
                set.insert(t);
            } else if set.contains(&t) && !rec.on {
                count -=1;
                set.remove(&t);
            }
        }
    }
    count
}

fn main() {

    println!("{}",part_a("data/day22.txt"));
    println!("{}",part2("data/day22.txt"));
}


