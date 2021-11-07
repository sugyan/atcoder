use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut v = Vec::with_capacity(n);
    for _ in 0..n {
        input! {
            t: u64,
            a: [usize],
        }
        v.push((t, a));
    }
    let mut answer = v[n - 1].0;
    let mut learned = vec![false; n];
    let mut stack = v[n - 1].1.clone();
    while let Some(last) = stack.pop() {
        if learned[last - 1] {
            continue;
        }
        learned[last - 1] = true;
        answer += v[last - 1].0;
        stack.extend(v[last - 1].1.iter().cloned());
    }
    println!("{}", answer);
}
