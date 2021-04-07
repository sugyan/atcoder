use proconio::marker::Chars;
use proconio::{fastout, input};
use std::collections::VecDeque;

#[fastout]
fn main() {
    input! {
        mut a: [Chars; 10],
    }
    let check = |(i, j): (usize, usize)| -> bool {
        let mut a = a.clone();
        let mut vd = VecDeque::new();
        vd.push_back((i, j));
        while let Some((i, j)) = vd.pop_front() {
            for &(di, dj) in &[(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let (i, j) = (i as i32 + di, j as i32 + dj);
                if (0..10).contains(&i) && (0..10).contains(&j) {
                    let (i, j) = (i as usize, j as usize);
                    if a[i][j] == 'o' {
                        a[i][j] = 'x';
                        vd.push_back((i, j));
                    }
                }
            }
        }
        a.iter().all(|row| row.iter().all(|&c| c == 'x'))
    };
    let can_unify = || -> bool {
        for (i, row) in a.iter().enumerate() {
            for (j, &col) in row.iter().enumerate() {
                if col == 'x' && check((i, j)) {
                    return true;
                }
            }
        }
        false
    };
    if can_unify() {
        println!("YES");
    } else {
        println!("NO");
    }
}
