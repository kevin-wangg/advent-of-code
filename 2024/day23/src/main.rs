use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;

fn bron_kerbosch(
    r: HashSet<String>,
    p: HashSet<String>,
    mut x: HashSet<String>,
    adj_list: &HashMap<&str, Vec<&str>>,
    cliques: &mut Vec<HashSet<String>>,
) {
    if p.is_empty() && x.is_empty() {
        cliques.push(r.clone());
        return;
    }

    for node in p.iter() {
        let mut new_r = r.clone();
        new_r.insert(node.clone());
        let mut new_p = HashSet::new();
        let mut new_x = HashSet::new();
        for possible in p.iter() {
            if !x.contains(possible) && adj_list[&node.as_ref()].contains(&possible.as_ref()) {
                new_p.insert(possible.clone());
            }
        }
        for considered in x.iter() {
            if adj_list[&node.as_ref()].contains(&considered.as_ref()) {
                new_x.insert(considered.clone());
            }
        } 
        bron_kerbosch(new_r, new_p, new_x, adj_list, cliques);

        x.insert(node.clone());
    }
}

fn solve(input: &str) {
    let mut adj_list: HashMap<&str, Vec<&str>> = HashMap::new();

    for line in input.lines() {
        let nodes: Vec<&str> = line.split('-').collect();
        adj_list
            .entry(nodes[0])
            .or_insert(Vec::new())
            .push(nodes[1]);
        adj_list
            .entry(nodes[1])
            .or_insert(Vec::new())
            .push(nodes[0]);
    }

    let all_nodes: HashSet<_> = adj_list.keys().map(|k| k.to_string()).clone().collect();

    let r = HashSet::new();
    let p = all_nodes;
    let x = HashSet::new();
    let mut cliques = Vec::new();

    bron_kerbosch(r, p, x, &adj_list, &mut cliques);

    let mut ans = String::new();
    let mut max_len = 0;

    for clique in cliques {
        if clique.len() > max_len {
            let mut clique = clique.into_iter().collect::<Vec<_>>();
            clique.sort();
            ans = clique.join(",");
            max_len = clique.len();
        }
    }
    
    println!("ans {}", ans);
}

fn main() {
    let path = env::args().nth(1).unwrap_or("input.txt".to_string());
    let input = fs::read_to_string(path).expect("File should exist");
    solve(&input);
}
