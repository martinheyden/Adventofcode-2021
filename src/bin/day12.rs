use aoc_2021::read_input;
use regex::Regex;
use std::collections::HashMap;

fn main() {
    println!("{}", problem_a("data/day12.txt"));
    println!("{}", problem_b("data/day12.txt"));
}

struct CaveNetwork {
    neighbour_vec: Vec<Vec<usize>>,
    big_cave_vec: Vec<bool>,
    start_node: usize,
    end_node: usize,
    node_dict: HashMap<String, usize>,
    nbr_nodes: usize,
}

impl CaveNetwork {
    fn new(edge_str: &Vec<String>) -> CaveNetwork {
        let mut cn = CaveNetwork {
            neighbour_vec: Vec::new(),
            big_cave_vec: Vec::new(),
            start_node: 0,
            end_node: 0,
            node_dict: HashMap::new(),
            nbr_nodes: 0,
        };
        let re = Regex::new(r"^(\w+)-(\w+)$").unwrap(); //A bit overkill maybe...
        let mut n = [0, 0];
        for line in edge_str {
            let c = re.captures(line).unwrap();
            for i in 0..2 {
                n[i] = cn.get_number(&c[i + 1]);
            }
            cn.neighbour_vec[n[0]].push(n[1]);
            cn.neighbour_vec[n[1]].push(n[0]);
        }
        cn
    }

    fn get_number(&mut self, node_name: &str) -> usize {
        match self.node_dict.get(node_name) {
            Some(i) => return *i,
            None => {
                self.node_dict.insert(node_name.to_string(), self.nbr_nodes); //Add to map from name ->number
                self.big_cave_vec
                    .push(node_name.chars().next().unwrap().is_uppercase()); //check if big or not
                if node_name == "start" {
                    //Check if start
                    self.start_node = self.nbr_nodes;
                } else if node_name == "end" {
                    //Or end
                    self.end_node = self.nbr_nodes;
                }
                self.neighbour_vec.push(Vec::new()); //Add new and empty neighbour vec
                self.nbr_nodes += 1; //First node is node 0, so we increment here
                return self.nbr_nodes - 1;
            }
        }
    }
}

fn problem_a(file_name: &str) -> usize {
    let data = read_input::read_file_to_string_vec(file_name);
    let cn = CaveNetwork::new(&data);
    let mut visited = vec![false; cn.neighbour_vec.len()];
    visited[cn.start_node] = true;
    count_paths(cn.start_node, &cn, visited, true)
}

fn problem_b(file_name: &str) -> usize {
    let data = read_input::read_file_to_string_vec(file_name);
    let cn = CaveNetwork::new(&data);
    let mut visited = vec![false; cn.neighbour_vec.len()];
    visited[cn.start_node] = true;
    count_paths(cn.start_node, &cn, visited, false)
}

fn count_paths(from: usize, cn: &CaveNetwork, mut visited: Vec<bool>, double_done: bool) -> usize {
    if from == cn.end_node {
        return 1;
    } else {
        if !cn.big_cave_vec[from] {
            visited[from] = true; //If not big, set as visited
        }
        let mut count = 0;
        for dest in cn.neighbour_vec[from].iter() {
            if !visited[*dest] {
                count += count_paths(*dest, cn, visited.clone(), double_done);
            } else if !double_done && *dest != cn.start_node {
                count += count_paths(*dest, cn, visited.clone(), true);
            }
        }
        return count; //Todo, possible to make tail recursive?
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(226, problem_a("data/day12_test.txt"))
    }

    #[test]
    fn part2() {
        assert_eq!(3509, problem_b("data/day12_test.txt"))
    }
}
