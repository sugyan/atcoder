use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: [String; 3],
    }
    let mut ss = s.join("").chars().collect::<Vec<_>>();
    ss.sort_unstable();
    ss.dedup();
    if ss.len() > 10 || s[2].len() > s[0].len().max(s[1].len()) + 1 {
        println!("UNSOLVABLE");
        return;
    }
    let s = s
        .iter()
        .map(|s| {
            s.as_bytes()
                .iter()
                .rev()
                .map(|u| (u - b'a') as usize)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut v = [None; 26];
    if backtrack(&s, &mut v, 0, false) {
        for s in s {
            println!(
                "{}",
                s.iter()
                    .rev()
                    .map(|&u| (v[u].unwrap() + b'0') as char)
                    .collect::<String>()
            );
        }
    } else {
        println!("UNSOLVABLE");
    }
}

fn backtrack(s: &[Vec<usize>], v: &mut [Option<u8>; 26], i: usize, carry: bool) -> bool {
    if i == s[2].len() {
        return v[s[2][i - 1]] != Some(0) && !carry;
    }
    let d0 = if let Some(&u) = s[0].get(i) {
        if let Some(d) = v[u] {
            d
        } else {
            for j in 0..10 {
                if v.contains(&Some(j)) || (i == s[0].len() - 1 && j == 0) {
                    continue;
                }
                v[u] = Some(j);
                if backtrack(s, v, i, carry) {
                    return true;
                }
                v[u] = None;
            }
            return false;
        }
    } else {
        0
    };
    let d1 = if let Some(&u) = s[1].get(i) {
        if let Some(d) = v[u] {
            d
        } else {
            for j in 0..10 {
                if v.contains(&Some(j)) || (i == s[1].len() - 1 && j == 0) {
                    continue;
                }
                v[u] = Some(j);
                if backtrack(s, v, i, carry) {
                    return true;
                }
                v[u] = None;
            }
            return false;
        }
    } else {
        0
    };
    let d2 = d0 + d1 + if carry { 1 } else { 0 };
    if let Some(d) = v[s[2][i]] {
        return if d == d2 % 10 {
            backtrack(s, v, i + 1, d2 > 9)
        } else {
            false
        };
    } else if v.contains(&Some(d2 % 10)) {
        return false;
    }
    v[s[2][i]] = Some(d2 % 10);
    if backtrack(s, v, i + 1, d2 > 9) {
        return true;
    }
    v[s[2][i]] = None;
    false
}
