// CSES Sorting & Searching Q-20 :: Nested Ranges Check
// DateSolved: 23 April 2026
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
struct FenwickTree {
    a: Vec<usize>,
    bit: Vec<usize>,
    n: usize,
    total_sum: usize,
}

impl FenwickTree {
    fn new(a: Vec<usize>) -> Self {
        let n = a.len();
        let mut bit: Vec<usize> = vec![0; n];
        let mut total_sum = 0;
        for i in 0..n {
            let mut j_i = i;
            while j_i < n {
                bit[j_i] += a[i];
                j_i |= j_i + 1;
            }
            total_sum += a[i];
        }
        FenwickTree {
            a,
            bit,
            n,
            total_sum,
        }
    }
    fn update(&mut self, i: usize, new_val: usize) {
        let mut j_i = i;
        let old_val = self.a[i];
        while j_i < self.n {
            self.bit[j_i] += new_val;
            self.bit[j_i] -= old_val;
            j_i |= j_i + 1;
        }
        self.a[i] = new_val;
        self.total_sum += new_val;
        self.total_sum -= old_val;
    }
    fn sum(&self, mut i: usize) -> usize {
        let mut sum = 0;
        loop {
            sum += self.bit[i];
            let g_i = i & (i + 1);
            if g_i == 0 {
                break;
            }
            i = g_i - 1;
        }
        sum
    }
    /// returns the largest index with prefix_sum >= k.
    fn _binary_lift_search(&self, k: usize) -> usize {
        let mut current = 0;
        let mut sum = 0;
        for jump in (0..=self.n.ilog2()).rev() {
            let next_position = current + (1 << jump);
            if next_position > self.n {
                continue;
            }
            if sum + self.bit[next_position - 1] < k {
                sum += self.bit[next_position - 1];
                current = next_position;
            }
        }
        current
    }
}

fn main() {
    let mut cin = Scanner::default();
    let n: usize = cin.next();
    let mut ranges: Vec<(usize, usize, usize)> =
        (0..n).map(|i| (cin.next(), cin.next(), i)).collect();
    ranges.sort_unstable_by(|a, b| a.0.cmp(&b.0).then(b.1.cmp(&a.1)));
    // dbg!(&ranges);
    
    let mut unique_endpoints: Vec<usize> = ranges.iter().map(|r| r.1).collect();
    unique_endpoints.sort_unstable();
    unique_endpoints.dedup();

    let a = vec![0; unique_endpoints.len()];
    let mut fenwick = FenwickTree::new(a);
    let mut enclosed_by: Vec<usize> = vec![0; n];
    for i in 0..n {
        let current = ranges[i];
        // all the elements before it are candidates for enclosing current range
        // we need to efficiently find
        // just the number of all the elements in these who have endpoint >= current.1
        // for this we can query the sum in fenwick tree
        // but for that fenwick tree must be based on endpoints
        // all endpoints must be mapped to a unique sorted list.
        let rank = unique_endpoints.binary_search(&current.1).unwrap();
        if rank > 0 {
            enclosed_by[current.2] = fenwick.total_sum - fenwick.sum(rank - 1);
        } else {
            enclosed_by[current.2] = fenwick.total_sum;

        }
        fenwick.update(rank, fenwick.a[rank] + 1);
    }
    // dbg!(&fenwick, &enclosed_by);
    
    // we will repeat the same process but in reverse for
    // finding the ranges enclosed by our range

    for i in 0..fenwick.n {
        fenwick.update(i, 0);
    }

    let mut encloses: Vec<usize> = vec![0; n];

    for i in (0..n).rev() {
        let current = ranges[i];

        // we need to find number of elements with endpoints <= current.1
        let rank = unique_endpoints.binary_search(&current.1).unwrap();
        encloses[current.2] = fenwick.sum(rank);
        fenwick.update(rank, fenwick.a[rank] + 1);
    }

    // dbg!(&encloses);

    for i in encloses {
        if i > 0 {
            print!("1 ");
        } else {
            print!("0 ");
        }
    }
    println!();
    for i in enclosed_by {
        if i > 0 {
            print!("1 ");
        } else {
            print!("0 ");
        }
    }
}