use hashbrown::HashMap;



fn main() {
    let p1 = 1;
    let p2 = 2;
    println!("Part a ans:{}",play_game(p1,p2));
    let mut cache = HashMap::new();
    let res = play_game_quantom_2(1, 2, 0, 0,true, &mut cache);
    if res.0>res.1 {
        println!{"Part b  ans: {}", res.0};
    }else {
        println!{"Part b  ans: {}", res.1};
    }
}

fn move_player(p: usize, mv: usize) -> usize {
    (p+mv-1)%10 +1
}

fn roll_die(old_val: &mut usize) {
    *old_val = (*old_val)%100 +1
}

fn roll_die_three_times(old_val: &mut usize) ->usize {
    let mut agg_roll = 0;
    for _ in 0..3 {
        roll_die(old_val);
        agg_roll += *old_val;
    }
    agg_roll
}

fn play_game(p1:usize,p2:usize) ->usize {
    let mut score_1 = 0;
    let mut score_2 = 0;
    let mut die_val = 0;
    let mut nbr_rolls = 0;
    let mut p1 = p1;
    let mut p2 = p2;
    loop {
        let agg_roll = roll_die_three_times(&mut die_val);
        p1 = move_player(p1,agg_roll);
        score_1+=p1;
        nbr_rolls+=3;
        if score_1>=1000 {
            return nbr_rolls*score_2;
        }
        let agg_roll = roll_die_three_times(&mut die_val);
        p2 = move_player(p2,agg_roll);
        score_2+=p2;
        nbr_rolls+=3;
        if score_2>=1000 {
            return nbr_rolls*score_1;
        }
    }
}

fn get_number_universes(v:usize) -> usize {
    match v {
        3 => 1,
        4 => 3,
        5 => 6,
        6 => 7,
        7 => 6,
        8 => 3,
        9 => 1,
        _ => panic!("can roll that"),
    }
}



fn play_game_quantom_2(p1:usize,p2:usize,score_1:usize,score_2:usize,player_1_turn: bool, cache: &mut HashMap<isize,(usize,usize)>) ->(usize,usize) {
    if score_1 >= 21 { //P1 wins first
        return (1,0)
    }
    if score_2 >=21 {
        return (0,1)
    }
    match cache.get(&hash(p1,p2,score_1,score_2,player_1_turn)) {
        Some(v) => *v,
        None => {
            let mut agg_res = (0,0);
            for m in 3..10{
                if player_1_turn {
                    let p1_l = move_player(p1, m);
                    let res = play_game_quantom_2(p1_l, p2, score_1+p1_l, score_2,false,cache);
                    agg_res.0 += res.0*get_number_universes(m);
                    agg_res.1 += res.1*get_number_universes(m);
                } else {
                    let p2_l = move_player(p2, m);
                    let res = play_game_quantom_2(p1, p2_l, score_1, score_2+p2_l,true,cache);
                    agg_res.0 += res.0*get_number_universes(m);
                    agg_res.1 += res.1*get_number_universes(m);
                }
            }
            cache.insert(hash(p1,p2,score_1,score_2,player_1_turn), agg_res);
            agg_res
        }
    }
}

fn hash(p1:usize,p2:usize,score_1:usize,score_2:usize,player_1_turn: bool) ->isize {
    let val = p1+p2*10 + score_1*100+score_2*10000;
    if player_1_turn {
        val as isize
    } else {
        -(val as isize)
    }
}