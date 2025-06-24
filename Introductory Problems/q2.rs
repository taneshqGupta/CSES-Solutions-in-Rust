// CSES ProblemSet Q-2 :: Missing Number
// DateSolved: 24 Jun 2025
// SolvedBy: taneshqGupta

use crate::helpers::*;
use std::io::{Read, Write};
 
fn solve<'a>(cin: &mut impl Iterator<Item = &'a str>, cout: &mut impl Write) -> Option<()> {
 
    let n: usize = get(cin)?;
 
    const N: usize = 3e5 as usize;
 
    let mut aa: [usize; N] = [0; N];
 
    let mut ans = 0;
 
    for _ in 0..n-1 {
        let curr: usize = get(cin)?;
        aa[curr] += 1;
    }    
 
    for i in 1..=n{
        if aa[i] == 0 { ans = i; }
    }    
 
    set(cout, &ans); nl(cout);
 
    Some(())
}
 
#[allow(dead_code)]
mod helpers {
    use std::{fmt::Display, io::Write, str::FromStr};
 
    pub fn set<T: Display>(cout: &mut impl Write, a: T) {
        write!(cout, "{}", a).ok();
    }
 
    pub fn get<'a, T: FromStr>(cin: &mut impl Iterator<Item = &'a str>) -> Option<T> {
        Some(cin.next()?.parse::<T>().ok()?)
    }
 
    pub fn get_vec<'a, T: FromStr>(
        cin: &mut impl Iterator<Item = &'a str>,
        n: usize,
    ) -> Option<Vec<T>> {
        Some(cin.take(n).filter_map(|s| s.parse::<T>().ok()).collect())
    }
 
    pub fn set_vec<T: Display>(cout: &mut impl Write, aa: &[T]) {
        let mut iter = aa.iter().peekable();
        while let Some(item) = iter.next() {
            write!(cout, "{}", item).ok();
            if iter.peek().is_some() {
                write!(cout, " ").ok();
            }
        }
    }
 
    pub fn nl(cout: &mut impl Write) {
        writeln!(cout, "").ok();
    }
 
    pub fn sp(cout: &mut impl Write) {
        write!(cout, " ").ok();
    }
}
 
fn main() {
    let mut cout = std::io::BufWriter::new(std::io::stdout());
    let mut s = String::new();
    std::io::stdin().read_to_string(&mut s).unwrap();
    let mut cin = s.split_ascii_whitespace();
 
    // -- if there are test cases --
    // let _t: usize = get(&mut cin).unwrap();
    // -- if there are test cases --
 
    while let Some(_) = solve(&mut cin, &mut cout) {}
}