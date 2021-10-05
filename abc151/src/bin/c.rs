use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize, m: usize,
        ps: [(usize, String); m],
    }
    let (mut ac, mut wa) = (vec![false; n], vec![0; n]);
    for (p, s) in &ps {
        match s.as_str() {
            "AC" => ac[*p - 1] = true,
            "WA" if !ac[*p - 1] => wa[*p - 1] += 1,
            _ => {}
        }
    }
    let answer = (
        ac.iter().filter(|&b| *b).count(),
        (0..n).map(|i| if ac[i] { wa[i] } else { 0 }).sum::<i32>(),
    );
    println!("{} {}", answer.0, answer.1);
}
