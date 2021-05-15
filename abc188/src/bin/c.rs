use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [u32; 1 << n],
    }
    let mut v = (0..1 << n).collect::<Vec<_>>();
    for i in 1..n {
        for j in 0..1 << (n - i) {
            let lhs = j * (1 << i);
            let rhs = lhs + (1 << (i - 1));
            v[lhs] = if a[v[lhs]] > a[v[rhs]] {
                v[lhs]
            } else {
                v[rhs]
            };
        }
    }
    let answer = if a[v[0]] < a[v[1 << (n - 1)]] {
        v[0]
    } else {
        v[1 << (n - 1)]
    } + 1;
    println!("{}", answer);
}
