use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut ab: [(i64, i64); n],
    }
    ab.sort_by_cached_key(|(a, b)| a + b);
    ab.reverse();
    let mut answer = 0;
    for (i, (a, b)) in ab.iter().enumerate() {
        if i % 2 == 0 {
            answer += a + b;
        }
        answer -= b;
    }
    println!("{}", answer);
}
