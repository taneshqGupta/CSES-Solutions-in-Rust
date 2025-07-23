// CSES ProblemSet Q-2 :: Hidden Permutation
// DateSolved: 23 Jul 2025
// SolvedBy: taneshqGupta

use std::collections::BTreeMap;

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

struct Node {
    key: usize,
    height: usize,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(key: usize) -> Self {
        Node {
            key,
            height: 1,
            left: None,
            right: None,
        }
    }
}

struct AVL {
    root: Option<Box<Node>>,
    cin: Scanner,
}

impl AVL {
    fn new() -> Self {
        AVL {
            root: None,
            cin: Scanner::default(),
        }
    }

    fn height(node: &Option<Box<Node>>) -> usize {
        if let Some(n) = node {
            n.height
        } else {
            0
        }
    }

    fn balance_factor(node: &Option<Box<Node>>) -> isize {
        if let Some(n) = node {
            Self::height(&n.left) as isize - Self::height(&n.right) as isize
        } else {
            0
        }
    }

    fn balance_factor_box(node: &Box<Node>) -> isize {
        Self::height(&node.left) as isize - Self::height(&node.right) as isize
    }

    fn update_height(node: &mut Box<Node>) {
        node.height = 1 + std::cmp::max(Self::height(&node.left), Self::height(&node.right));
    }

    fn rotate_right(mut p: Box<Node>) -> Box<Node> {
        let mut q = p.left.take().unwrap();
        p.left = q.right.take();
        Self::update_height(&mut p);
        q.right = Some(p);
        Self::update_height(&mut q);
        q
    }

    fn rotate_left(mut p: Box<Node>) -> Box<Node> {
        let mut q = p.right.take().unwrap();
        p.right = q.left.take();
        Self::update_height(&mut p);
        q.left = Some(p);
        Self::update_height(&mut q);
        q
    }

    fn rotate_left_right(mut p: Box<Node>) -> Box<Node> {
        p.left = Some(Self::rotate_left(p.left.take().unwrap()));
        Self::rotate_right(p)
    }

    fn rotate_right_left(mut p: Box<Node>) -> Box<Node> {
        p.right = Some(Self::rotate_right(p.right.take().unwrap()));
        Self::rotate_left(p)
    }

    fn rebalance(mut p: Box<Node>) -> Box<Node> {
        let bf = Self::balance_factor_box(&p);
        
        if bf > 1 {
            if Self::balance_factor(&p.left) >= 0 {
                p = Self::rotate_right(p);
            } else {
                p = Self::rotate_left_right(p);
            }
        } else if bf < -1 {
            if Self::balance_factor(&p.right) <= 0 {
                p = Self::rotate_left(p);
            } else {
                p = Self::rotate_right_left(p);
            }
        }
        
        Self::update_height(&mut p);
        p
    }

    fn is_smaller(&mut self, x: usize, y: usize) -> bool {
        println!("? {} {}", x, y);
        let s: String = self.cin.next();
        s == "YES"
    }

    fn insert_node(&mut self, node: Option<Box<Node>>, key: usize) -> Option<Box<Node>> {
        match node {
            None => Some(Box::new(Node::new(key))),
            Some(mut p) => {
                if self.is_smaller(key, p.key) {
                    p.left = self.insert_node(p.left.take(), key);
                } else {
                    p.right = self.insert_node(p.right.take(), key);
                }
                Some(Self::rebalance(p))
            }
        }
    }

    fn insert(&mut self, key: usize) {
        let root = self.root.take();
        self.root = self.insert_node(root, key);
    }

    fn inorder_traversal(node: &Option<Box<Node>>, result: &mut Vec<usize>) {
        if let Some(n) = node {
            Self::inorder_traversal(&n.left, result);
            result.push(n.key);
            Self::inorder_traversal(&n.right, result);
        }
    }

    fn inorder(&self) -> Vec<usize> {
        let mut result = Vec::new();
        Self::inorder_traversal(&self.root, &mut result);
        result
    }
}

fn main() {
    let mut tree = AVL::new();
    let n: usize = tree.cin.next();
    
    for i in 1..=n {
        tree.insert(i);
    }
    
    let a = tree.inorder();
    let mut mapx = BTreeMap::new();
    
    for i in 0..n {
        mapx.insert(a[i], i + 1);
    }
    
    print!("! ");
    for i in mapx.values() {
        print!("{} ", i);
    }
    println!();
}
