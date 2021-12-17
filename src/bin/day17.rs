



fn n_fac(n:isize) ->isize { 
    (n+1)/2*n
}

//Resturns a vector containting vectors of time instances within target 
//for different initial velocities in y dirr
fn find_possible_y(ymin:isize,ymax:isize) -> Vec<Vec<isize>> {
    let mut v0_vec = Vec::new();
    for i in (-ymax-1)..=(-ymin-1) {
         v0_vec.push(vec![2*i+2]);
    }
    for v0 in ymin..-ymax-1 {
        let mut v = v0;
        let mut y = 0;
        let mut t_vec = Vec::new();
        let mut t=0;
        if v0>0 { //Can start at y= 0 second time
            v = -v0-1;
            t = 2*v0+1
        }
        while y >ymin{
            t = t+1;
            y = y+v;
            v = v-1;
            if y >=ymin && y<=ymax {
                t_vec.push(t);
            }
        }
        if !t_vec.is_empty() {
            v0_vec.push(t_vec);
        }
    }
    v0_vec
}


fn count_possible_initial(xmin:isize,xmax:isize,v0y_vec:&Vec<Vec<isize>>) ->usize {
    let mut count = 0;
    for v in v0y_vec {
        count += nbr_v0(&v,xmin,xmax);
    }
    count
}


// Find the number of v0 in x dirr that is within the target for at least one t in t_targets
fn nbr_v0(t_targets: &Vec<isize>,xmin:isize,xmax:isize) -> usize {
    let mut count = 0;
    let mut start = 0;
    for start_cand in 1..25 { //find a suitable starting position (a bit unecessary to do this every iter)
        if n_fac(start_cand)>xmin {
            start = start_cand;
            break;
        }
    }
    for v0 in start..= xmax {
        for t in t_targets {
            let end_pos = get_end_x_position(*t, v0);
            if end_pos>= xmin && end_pos <= xmax {
                count+=1;
                break;
            }
        }
    }
    count
}

fn get_end_x_position(t:isize, v0:isize) ->isize {
    if v0< t{
        return n_fac(v0);
    } else {
        return (v0+v0-t+1)*t/2; //rounding errors possible?
    }

}

fn problem_b(xmin:isize,xmax:isize,ymin:isize,ymax:isize) ->usize {
    count_possible_initial(xmin, xmax, &find_possible_y(ymin,ymax))
}

//ymin negative (and also ymax)
fn problem_a(ymin:isize) ->isize {
    n_fac(-ymin-1)
}

fn main() {
    println!("{}", problem_a(-198));
    println!("{}", problem_b(57,116,-198,-148));
}

