// CSES Introductory-Problems Q-24 :: Grid Path Description
// DateSolved: 8 April 2026
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
const MOVES: [(i8, i8); 4] = [(1, 0), (0, -1), (-1, 0), (0, 1)];

#[inline(always)]
fn is_visited(position: (i8, i8), visited: u64) -> bool {
    let (i, j) = position;
    if i < 0 || i > 6 || j < 0 || j > 6 {
        return true;
    }
    return (visited >> (i * 7 + j)) & 1 == 1;
}

fn dfs(
    forced_moves: &[Option<usize>; 48],
    ans_count: &mut usize,
    move_counter: usize,
    visited: u64,
    position: (i8, i8),
) {
    if position == (6, 0) && move_counter == 48 {
        *ans_count += 1;
        return;
    } else if position == (6, 0) {
        return;
    }

    // most important point to learn from this question
    // if we do not do the following filter,
    // the question is unsolvable for N=7.
    // Crucial as fuck:
    /*  Everytime we get to a wall OR a visited square (creating a loop),
        AND we can go both left or right (pending points on both sides),
        then, it is impossible to visit all points without crossing the
        path atleast once.
    */
    // So once this exact situation occurs, we must roll back and return.

    let (i, j) = position;
    let down_blocked = is_visited((i + 1, j), visited);
    let up_blocked = is_visited((i - 1, j), visited);
    let right_blocked = is_visited((i, j + 1), visited);
    let left_blocked = is_visited((i, j - 1), visited);

    if (down_blocked && up_blocked && !left_blocked && !right_blocked)
        || (left_blocked && right_blocked && !up_blocked && !down_blocked)
    {
        return;
    }

    let forced = forced_moves[move_counter];
    let (start_dir, end_dir) = if let Some(dir) = forced {
        (dir, dir)
    } else {
        (0, 3)
    };

    for i in start_dir..=end_dir {
        let new_position = (
            position.0 + MOVES[i].0,
            position.1 + MOVES[i].1,
        );
        if new_position.0 < 0 || new_position.0 > 6 || new_position.1 < 0 || new_position.1 > 6 {
            continue;
        }
        if is_visited(new_position, visited) {
            continue;
        }
        if !is_visited(new_position, visited) {
            let (i, j) = new_position;
            let next_visited = visited | 1 << (i * 7 + j);
            dfs(
                forced_moves,
                ans_count,
                move_counter + 1,
                next_visited,
                new_position,
            );
        }
    }
}

fn main() {
    let mut cin = Scanner::default();
    let s: Vec<char> = cin.next::<String>().chars().collect();
    // Let :: D = 0, L = 1, U = 2, R = 3
    let mut forced_moves: [Option<usize>; 48] = [None; 48];
    for i in 0..48 {
        if s[i] != '?' {
            let insertable: usize = match s[i] {
                'D' => 0,
                'L' => 1,
                'U' => 2,
                'R' => 3,
                _ => 4,
            };
            forced_moves[i] = Some(insertable);
        }
    }
    let mut ans_count: usize = 0;

    let visited = 1;
    dfs(&forced_moves, &mut ans_count, 0, visited, (0, 0));

    println!("{}", ans_count);
}