// CSES Graph-Algorithms Q-5 :: Road Construction
// DateSolved: 19 Sep 2025
// SolvedBy: taneshqGupta

use std::array;

#[derive(Default, Debug)]
struct Scanner(Vec<String>);
impl Scanner {
    fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(c) = self.0.pop() {
                return c.parse().ok().unwrap();
            }
            let mut s = String::new();
            std::io::stdin().read_line(&mut s).unwrap();
            self.0 = s.split_whitespace().rev().map(String::from).collect();
        }
    }
}

fn main() {
    let mut cin = Scanner::default();
    let [n, m] = array::from_fn(|_| cin.next::<usize>());
    let mut parent: Vec<usize> = (0..n + 1).map(|i| i).collect();
    let mut size = vec![1; n + 1];
    fn find(parent: &mut Vec<usize>, v: usize) -> usize {
        let mut curr = v;
        while parent[curr] != curr {
            curr = parent[curr];
        }
        let root = curr;
        let mut curr = v;
        while parent[curr] != root {
            let next = parent[curr];
            parent[curr] = root;
            curr = next;
        }
        root
    }
    fn union(parent: &mut Vec<usize>, size: &mut Vec<usize>, u: usize, v: usize) -> Option<usize> {
        let u_root = find(parent, u);
        let v_root = find(parent, v);
        if u_root == v_root {
            return None;
        }
        if size[u_root] < size[v_root] {
            parent[u_root] = v_root;
            size[v_root] += size[u_root];
            Some(size[v_root])
        } else {
            parent[v_root] = u_root;
            size[u_root] += size[v_root];
            Some(size[u_root])
        }
    }

    let mut edges = Vec::new();
    
    for _ in 0..m {
        let [u, v] = array::from_fn(|_| cin.next::<usize>());
        edges.push((u, v));
    }

    let mut num_conx = n;
    let mut largest_conx = 1;

    for &edge in &edges {
        if let Some(size) = union(&mut parent, &mut size, edge.0, edge.1) {
            num_conx -= 1;
            largest_conx = largest_conx.max(size);
        }
        println!("{} {}", num_conx, largest_conx);
    }
}