use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [u32; 1 << n],
    }
    let l = (0..1 << (n - 1)).max_by_key(|&i| a[i]).unwrap();
    let r = (1 << (n - 1)..1 << n).max_by_key(|&i| a[i]).unwrap();
    let answer = if a[l] > a[r] { r + 1 } else { l + 1 };
    println!("{}", answer);
}
