use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: [String; 3],
    }
    let mut v = [None; 26];
    let mut i = 0;
    let s = s
        .iter()
        .map(|s| {
            s.as_bytes()
                .iter()
                .map(|&u| {
                    *v[(u - b'a') as usize].get_or_insert_with(|| {
                        i += 1;
                        i - 1
                    })
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    if i <= 10 {
        for p in (0..10).permutations(i) {
            let n = s
                .iter()
                .map(|v| v.iter().fold(0_i64, |acc, &i| acc * 10 + p[i]))
                .collect::<Vec<_>>();
            if n[0] + n[1] == n[2] && p[s[0][0]] * p[s[1][0]] * p[s[2][0]] != 0 {
                for &n in &n {
                    println!("{}", n);
                }
                return;
            }
        }
    }
    println!("UNSOLVABLE");
}
