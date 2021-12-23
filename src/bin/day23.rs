use std::cmp::min;
use std::cmp::max;
use std::collections::HashMap;

struct State {
    s: [usize;27],
    // 0..10 corridor, 11-12 a, 13-14 b, 15-16 c, 17-18d
}

impl State{

    fn done(&self) -> bool {
        for i in 11..=14 {
            if self.s[i] != 1 {
                return false;
            }
        }
        for i in 15..=18 {
            if self.s[i] != 10 {
                return false;
            }
        }
        for i in 19..=22 {
            if self.s[i] != 100 {
                return false;
            }
        }
        for i in 23..=26 {
            if self.s[i] != 1000 {
                return false;
            }
        }
        true
    }

    fn valid_move(&self,source:usize,dest:usize) -> bool {
        let vals = [1,10,100,1000];
        //Moving nothing or moving into something
        if self.s[source] == 0 || self.s[dest] != 0{
            return false; 
        }
        //Invalid column destination
        if dest >=2 && dest<=8 && dest%2 == 0 {
            return false;
        }
        //If moving out of column but top is not empty
        for i in 0..4 { //for column
            for j in 1..=3 { //for bottom three positions
                if source == 11+i*4+j {
                    for k in 0..j{ //check above
                        if self.s[11+i*4+k] != 0 {
                            return false;
                        }
                    }
                }

            }
        }
        //if moving into bottom but top is not empty
        for i in 0..4 {
            for j in 1..=3 { //bottom three positions
                if dest == 11+i*4+j {
                    for k in 0..j{ //check above
                        if self.s[11+i*4+k] != 0 { //not empty
                            return false;
                        }
                    }
                }

            }
        }
        // Moving within  corridor
        if source<11 && dest <11 {
            return false;
        }
       // println!("source: {}  dest: {} possible",source,dest);
        //moving from room - not possible if correct and all bellow correct
        for i in 0..4 { // rooms
            for j in 0..=2 { //first three positions
                //if in correct room
                if source == 11+ i * 4 + j && self.s[source] ==vals[i]  {
                    let mut ilegal = true;
                    for k in (j+1)..=3 { //check all below
                        //if all correct, then cant move
                        if self.s[11 + i * 4 + k] != vals[i] {
                            ilegal = false ;
                        }
                    } 
                    if ilegal {
                      //  println!("source: {}  dest: {} ilegal",source,dest);
                        return false;
                    }
                }
            } // last position
            if source == 11+i*4+3 && self.s[source] == vals[i] {
                return false;
            }
        }
        //println!("source: {}  dest: {} still possible",source,dest);

        //Moving to room - only possible if all below correct
        for i in 0..4 { // rooms
            for j in 0..3 { //position in room
                //Wrong room
                if dest == 11+i*4+j && self.s[source] != vals[i] {
                    return false;
                }
                //Correct room
                if dest == 11+i*4 + j {
                    for k in j+1..=3 { //check all below
                        //if all correct, then can move
                        let mut ilegal = false;
                        if self.s[11+i*4+k] != vals[i] { //if incorrect below
                            ilegal = true; //then ilegal
                        }
                        if ilegal {
                            return false;
                        }
                    } 
                }
            } // last position, incorrect room
            if dest == 11+i*4+3 && self.s[source] != vals[i] {
                return false;
            }
        }



        //moving back and forth same pocket
        for i in 0..4 {
            let range = 11+4*i .. 11+4*(i+1);
            if range.contains(&source) && range.contains(&dest) {
                return false;
            }

        }
        //Check corridor isnt blocked
        for i in min(get_corridor_number(source),get_corridor_number(dest))+1..=
            max(get_corridor_number(source),get_corridor_number(dest))-1 {
            if self.s[i] != 0 {
                return false;
            }
        }
        return true;
    }

    fn get_distance(source:usize, dest:usize) -> usize{
        let mut dist = max(get_corridor_number(source),get_corridor_number(dest)) -
            min(get_corridor_number(source),get_corridor_number(dest));
        if source >=11 {
            dist+= (source-11)%4+1
        }
        if dest >=11 {
            dist += (dest-11)%4+1;
        }
        dist
    }

    fn make_move(&self,source:usize,dest:usize) -> (usize,State) {
        let cost = State::get_distance(source, dest)*self.s[source];
        let mut new_s = self.s.clone();
        new_s[dest] = new_s[source];
        new_s[source] = 0;
        (cost, State{s:new_s})
    }

}



fn get_corridor_number(location:usize) -> usize{
    if location == 11 || location == 12 || location == 13 || location ==14 {
        return 2;
    }
    if location == 15 || location == 16 || location == 17 || location ==18 {
        return 4;
    }
    if location == 19 || location == 20 || location == 21 || location ==22 {
        return 6;
    }
    if location == 23 || location == 24 || location == 25 || location ==26 {
        return 8;
    }
    if location <11 {
        return location;
    }
    panic!("Invalid location");
}


fn calc_cost(state: State, mem :&mut HashMap<String,usize>) -> usize {
    if state.done() {
       // println!("Done with {:?}", state.s);
        return 0;
    }   else if mem.contains_key(&hash(&state)) {
        return *mem.get(&hash(&state)).unwrap();
    } else {
        let mut best_cost = std::usize::MAX;
        //loop over possible moves
        for source in 0..27 {
            for dest in 0..27{
                if state.valid_move(source, dest) {
                 //   println!("testing move from {} to {} with state {:?}",source,dest,state.s);
                    let res = state.make_move(source, dest);
                    let cost_to_go = calc_cost(res.1,mem);

                    if cost_to_go != std::usize::MAX {
                        let new_cost = res.0 + cost_to_go;
                        if new_cost < best_cost {
                            best_cost = new_cost;
                        }
                    }

                }
            }
        }
        // if best_cost >= 1000*100 {
        //     return std::usize::MAX-1;
        // }
        mem.insert(hash(&state), best_cost);
        return best_cost;
    }
}


fn hash(state: &State) -> String {
    let mut s = String::new();
    for u  in state.s {
        s.push_str(u.to_string().as_str());
        s.push(',');
    }
    s
}

fn main() { 
    let mut map = HashMap::new();
    //let init = State{s:[0,0,0,0,0,0,0,0,0,0,0,10,100,10,1,1000,1,1000,100]};
    let init = State{s:[0,0,0,0,0,0,0,0,0,0,0,
            10,100,1,1,
            10,1,10,10,
            1000,1,100,100,
            1000,100,1000,1000]};
    println!("Part a{}",calc_cost(init,&mut map));
    let mut map = HashMap::new();
    let init = State{s:[0,0,0,0,0,0,0,0,0,0,0,
            10,1000,1000,100,
            10,100,10,1,
            1000,10,1,1,
            1000,1,100,100]};
    println!("Part b{}",calc_cost(init,&mut map))
}