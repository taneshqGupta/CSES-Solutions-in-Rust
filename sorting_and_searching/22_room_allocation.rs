// CSES Sorting & Searching Q-22 :: Room Allocation
// DateSolved: 25 April 2026
// SolvedBy: taneshqGupta

use std::{cmp::Reverse, collections::BinaryHeap};

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
    let n: usize = cin.next();
    let mut intervals: Vec<(usize, usize, usize)> =
        (0..n).map(|i| (cin.next(), cin.next(), i)).collect();
    intervals.sort_unstable();
    let mut rooms: Vec<usize> = vec![0; n];
    let mut heapx: BinaryHeap<(Reverse<usize>, usize)> = BinaryHeap::new();
    heapx.push((Reverse(intervals[0].1), 1));
    let mut num_rooms = 1;
    rooms[0] = 1;
    for i in 1..n {
        let (start, depart, index) = intervals[i];
        let (Reverse(earliest_depart), _id) = heapx.peek().unwrap();
        if start <= *earliest_depart {
            num_rooms += 1;
            heapx.push((Reverse(depart), num_rooms));
            rooms[index] = num_rooms;
        } else {
            let (Reverse(_earliest_depart), id) = heapx.pop().unwrap();
            heapx.push((Reverse(depart), id));
            rooms[index] = id;
        }
    }
    println!("{}", num_rooms);
    for i in rooms {
        print!("{} ", i);
    }
    println!();
}