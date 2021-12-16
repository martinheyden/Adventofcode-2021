use aoc_2021::read_input;


fn push_str(substr: &str,vec: &mut Vec<char>) {
    for ch in substr.chars() {
        vec.push(ch);
    }
}

fn hexa_to_binary(hexa: &str) -> Vec<char> {
    let mut bin_vec = Vec::new();
    for ch in hexa.chars() {
        match ch {
            '0' => push_str("0000",&mut bin_vec),
            '1' => push_str("0001",&mut bin_vec),
            '2' => push_str("0010",&mut bin_vec),
            '3' => push_str("0011",&mut bin_vec),
            '4' => push_str("0100",&mut bin_vec),
            '5' => push_str("0101",&mut bin_vec),
            '6' => push_str("0110",&mut bin_vec),
            '7' => push_str("0111",&mut bin_vec),
            '8' => push_str("1000",&mut bin_vec),
            '9' => push_str("1001",&mut bin_vec),
            'A' => push_str("1010",&mut bin_vec),
            'B' => push_str("1011",&mut bin_vec),
            'C' => push_str("1100",&mut bin_vec),
            'D' => push_str("1101",&mut bin_vec),
            'E' => push_str("1110",&mut bin_vec),
            'F' => push_str("1111",&mut bin_vec),
             _  => panic!("unknown hexadecimal number"),
         };
    }
    bin_vec
}

fn binary_to_dec(vec: &[char]) -> usize {
    let mut val = 0;
    let mut base = 1;
    for ch in vec.iter().rev() {
        if *ch == '1' {
            val += base;
        }
        base = base*2;
    }
    val
}

fn parse_package(vec: &[char]) -> (usize,usize) {
    
    let mut vs = binary_to_dec(&vec[0..3]); //Read three bits version number
    println!("reading package {:?} with version {}", to_string(vec), vs);
    let pack_type = binary_to_dec(&vec[3..6]); //Read three bits package type
    let mut ind = 6; //Start at index 6
    match pack_type {
        //literal package
        4 => {
            println!("Starting to read literal");
            let mut go_on = true;
            while go_on && ind<vec.len() {
                //Check if last chunk
                if vec[ind] == '0' {
                    go_on = false;
                }
                ind = ind + 5; //Read next 5 chunk
            }
            if ind %4 !=0 {
                //ind = ind + 4-ind%4;
            } 
            println!("read literal {:?}", to_string(&vec[0..ind]));
        },
        //Operator package
        _ => {          
            //Number of bits in subpackage(s) known  
            if vec[ind] =='0' {
                let len_subpackets = binary_to_dec(&vec[ind+1..ind+1+15]);
                println!{"Operator length type 0 with supbackets of len {} ", len_subpackets};
                let mut ind_sum =0;
                ind = ind+16; //one bit for package type, 15 for length of subpackets
                while ind_sum < len_subpackets && !only_zeros(&vec[ind+ind_sum..]){
                    let r = parse_package(&vec[ind+ind_sum..]);
                    vs += r.0;
                    ind_sum+=r.1;
                }
                ind = ind + len_subpackets;
            //Number of 11 bits subpackages are known
            } else {
                let number_subpackets = binary_to_dec(&vec[ind+1..ind+1+11]); //11 bits -> number subpackets
                println!{"Operator length type 1 with {} subpackets", number_subpackets};
                ind = ind + 1 + 11; //type + 11 bits for number of packets
                for i in 0..number_subpackets {
                    let r = parse_package(&vec[ind..]); 
                    vs += r.0;
                    ind += r.1;
                }
            }
        }
    }
    (vs,ind)
}

fn only_zeros(vec: &[char]) -> bool {
    for ch in vec {
        if *ch == '1'{
            return false;
        }
    }
    return true;
}

fn version_sum(hexa: &str) -> usize{
    let char_vec = hexa_to_binary(hexa);
    parse_package(&char_vec[..]).0
}

fn parse_val(hexa: &str) -> usize{
    let char_vec = hexa_to_binary(hexa);
    parse_package_2(&char_vec[..]).0
}

fn problem_a(file_name: &str) ->usize {
    version_sum(&read_input::read_file_to_string(file_name))

}

fn problem_b(file_name: &str) ->usize {
    parse_val(&read_input::read_file_to_string(file_name))
  

}

fn parse_package_2(vec: &[char]) -> (usize,usize) {
    let pack_type = binary_to_dec(&vec[3..6]); //Read three bits package type
    let mut ind = 6; //Start at index 
    let mut val = 0;
    match pack_type {
        4 => {
            let mut go_on = true;
            while go_on  {
                //Check if last chunk
                if vec[ind] == '0' {
                    go_on = false;
                }
                val = val *2*2*2*2 + binary_to_dec(&vec[ind+1..ind+5]);
                ind = ind + 5; //Read next 5 chunk
            }
        },
        //Operator package
        _ => {
            let f = match pack_type {
                0 => |x, y| x+y,
                1 => |x, y| x*y,
                2 => |x, y| {if x <y {x} else {y}},
                3 => |x, y| {if x >y {x} else {y}},
                5 => |x, y| {if x >y {1} else {0}},
                6 => |x, y| {if x <y {1} else {0}},
                7 => |x, y| {if x ==y {1} else {0}},
                _=> panic!("invalid package type"),
            };
            //Number of bits in subpackage(s) known  
            if vec[ind] =='0' {
                let len_subpackets = binary_to_dec(&vec[ind+1..ind+1+15]);
                let mut ind_sum =0;
                ind = ind+16; //one bit for package type, 15 for length of subpackets
                while ind_sum < len_subpackets && !only_zeros(&vec[ind+ind_sum..]){
                    let r = parse_package_2(&vec[ind+ind_sum..]);
                    if ind_sum == 0 {
                        val = r.0;
                    } else {
                        val = f(val,r.0);
                    }
                    ind_sum+=r.1;
                }
                ind = ind + len_subpackets;
            //Number of 11 bits subpackages are known
            } else {
                let number_subpackets = binary_to_dec(&vec[ind+1..ind+1+11]); //11 bits -> number subpackets
                ind = ind + 1 + 11; //type + 11 bits for number of packets
                for i in 0..number_subpackets {
                    let r = parse_package_2(&vec[ind..]); 
                    if i == 0 {
                        val = r.0;
                    } else {
                        val = f(val,r.0);
                    }
                    ind += r.1;
                }
            }
        }
    }
    (val,ind)
}

fn main() {
    println!("{}", problem_a("data/day16.txt"));
    println!("{}", problem_b("data/day16.txt"));
}

fn to_string(v: &[char]) -> String {
    v.iter().fold(String::new(), |mut acc, el| {
        acc.push(*el);
        acc
    })
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn version_id() {
        assert_eq!(6, parse_package(&hexa_to_binary("D2FE28")).0)
    }

    #[test]
    fn version_sum_test() {
        assert_eq!(16 , version_sum("8A004A801A8002F478"));
        assert_eq!(12,version_sum("620080001611562C8802118E34"));

    }

    #[test]
    fn parse_test() {
        assert_eq!(3, parse_val("C200B40A82"));
        assert_eq!(54, parse_val("04005AC33890"));
        assert_eq!(7, parse_val("880086C3E88112"));
        assert_eq!(9, parse_val("CE00C43D881120"));
        assert_eq!(1, parse_val("D8005AC2A8F0"));
        assert_eq!(0, parse_val("F600BC2D8F"));
        assert_eq!(0, parse_val("9C005AC2F8F0"));
        assert_eq!(1, parse_val("9C0141080250320F1802104A08"));
    }



}
