// CSES Sorting & Searching Q-18 :: Josephus Problem I
// DateSolved: 16 April 2026
// SolvedBy: taneshqGupta

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

#[derive(Debug)]
struct CircularList {
    // The index represents the current node's ID.
    // The value stored at that index is the ID of the next node.
    next_node: Vec<usize>,
    len: usize,
}

impl CircularList {
    fn new(n: usize) -> Self {
        let mut next_node = vec![0; n];
        for i in 0..n {
            next_node[i] = i + 1;
        }
        next_node[n - 1] = 0;
        Self { next_node, len: n}
    }

    fn solve(&mut self) {
        let mut curr = 0;
        while self.len > 0 {
            let to_delete = self.next_node[curr];
            print!("{} ", to_delete + 1);

            // Unlink the target node by bridging over it
            self.next_node[curr] = self.next_node[to_delete];
            self.len -= 1;

            // Advance to the next surviving node
            curr = self.next_node[curr];
        }
    }
}

fn main() {
    let mut cin = Scanner::default();
    let mut list = CircularList::new(cin.next());
    list.solve();
}