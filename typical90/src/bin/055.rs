use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize, p: u64, q: u64,
        a: [u64; n],
    }
    let mut answer = 0;
    for i in 0..n - 4 {
        let val = a[i];
        for j in i + 1..n - 3 {
            let val = val * a[j] % p;
            for k in j + 1..n - 2 {
                let val = val * a[k] % p;
                for l in k + 1..n - 1 {
                    let val = val * a[l] % p;
                    for m in l + 1..n {
                        if val * a[m] % p == q {
                            answer += 1;
                        }
                    }
                }
            }
        }
    }
    println!("{}", answer);
}
