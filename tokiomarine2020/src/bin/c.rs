use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize, k: usize,
        mut a: [i32; n],
    }
    for _ in 0..k {
        let mut v = vec![0; n + 1];
        for (&a, i) in a.iter().zip(0..) {
            v[(i - a).max(0) as usize] += 1;
            v[(i + a + 1).min(n as i32) as usize] -= 1;
        }
        for i in 1..=n {
            v[i] += v[i - 1];
        }
        v.pop();
        if a.iter().all(|&a| a == n as i32) {
            break;
        }
        a = v;
    }
    let answer = a.iter().map(|i| i.to_string()).collect::<Vec<_>>();
    println!("{}", answer.join(" "));
}
