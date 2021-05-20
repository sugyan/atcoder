use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize, mut x: u64, m: u64,
    }
    let mut v = vec![None; m as usize];
    let mut sum = 0;
    let mut i = 0;
    while i < n {
        sum += x;
        if let Some((j, prev)) = v[x as usize] {
            let y = (n - i - 1) / (i - j);
            i += y * (i - j);
            sum += y as u64 * (sum - prev);
        }
        v[x as usize] = Some((i, sum));
        x = (x * x) % m;
        i += 1;
    }
    println!("{}", sum);
}
