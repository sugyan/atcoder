use proconio::marker::Chars;
use proconio::{fastout, input};
use std::collections::{HashSet, VecDeque};

#[fastout]
fn main() {
    input! {
        mut a: [Chars; 10],
    }
    let mut candidates = Vec::new();
    for i in 0..10 {
        for j in 0..10 {
            if a[i][j] == 'o' {
                let mut hs = HashSet::new();
                let mut vd = VecDeque::new();
                vd.push_back((i, j));
                while let Some((i, j)) = vd.pop_front() {
                    for &(di, dj) in &[(-1, 0), (1, 0), (0, -1), (0, 1)] {
                        let (i, j) = (i as i32 + di, j as i32 + dj);
                        if (0..10).contains(&i) && (0..10).contains(&j) {
                            let (i, j) = (i as usize, j as usize);
                            match a[i][j] {
                                'o' => {
                                    a[i][j] = '#';
                                    vd.push_back((i, j));
                                }
                                'x' => {
                                    hs.insert((i, j));
                                }
                                _ => {}
                            }
                        }
                    }
                }
                candidates.push(hs);
            }
        }
    }
    let intersection = |acc: Option<HashSet<_>>, x: &HashSet<_>| {
        Some(acc.map_or(x.clone(), |a| a.intersection(x).cloned().collect()))
    };
    if candidates
        .iter()
        .fold(None, intersection)
        .filter(|s| !s.is_empty())
        .is_some()
    {
        println!("YES");
    } else {
        println!("NO");
    }
}
