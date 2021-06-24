use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: u64,
    }
    for i in (1..).take_while(|&i| 3_u64.pow(i) < n) {
        let m = n - 3_u64.pow(i);
        if let Some(j) = (1..)
            .take_while(|&j| 5_u64.pow(j) <= m)
            .find(|&j| 5_u64.pow(j) == m)
        {
            println!("{} {}", i, j);
            return;
        }
    }
    println!("-1");
}
