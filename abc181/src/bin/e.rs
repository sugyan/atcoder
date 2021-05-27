use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize, m: usize,
        mut h: [i32; n],
        mut w: [i32; m],
    }
    h.sort_unstable();
    let mut sum = (vec![0; n / 2 + 1], vec![0; n / 2 + 1]);
    for i in 0..n / 2 {
        sum.0[i + 1] = sum.0[i] + h[i * 2 + 1] - h[i * 2];
    }
    for i in (0..n / 2).rev() {
        sum.1[i] = sum.1[i + 1] + h[i * 2 + 2] - h[i * 2 + 1];
    }
    let answer = w
        .iter()
        .map(|w| {
            let i = match h.binary_search(w) {
                Ok(i) => i,
                Err(i) => i,
            } & !1;
            sum.0[i / 2] + sum.1[i / 2] + (h[i] - w).abs()
        })
        .min()
        .unwrap();
    println!("{}", answer);
}
