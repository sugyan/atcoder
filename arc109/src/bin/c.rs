use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        _n: usize, k: usize,
        s: String,
    }
    let mut v = s
        .as_bytes()
        .iter()
        .map(|&u| match u {
            b'R' => 0,
            b'P' => 1,
            b'S' => 2,
            _ => unreachable!(),
        })
        .collect::<Vec<_>>();
    for _ in 0..k {
        v = v
            .repeat(v.len() % 2 + 1)
            .chunks(2)
            .map(|w| {
                if w[0] == w[1] {
                    w[0]
                } else {
                    w[(3 + w[0] - w[1]) % 3 - 1]
                }
            })
            .collect();
    }
    println!("{}", ['R', 'P', 'S'][v[0]]);
}
