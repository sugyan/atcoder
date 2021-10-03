use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut v = vec![0; n + 1];
    for mut i in 2..=n {
        while i > 1 {
            if let Some(j) = (2..=i).find(|&j| i % j == 0) {
                v[j] += 1;
                i /= j;
            }
        }
    }
    let mut answer = 0;
    for i in 1..=n {
        if v[i] >= 74 {
            answer += 1;
        }
        for j in (1..=n).filter(|&j| j != i) {
            if v[i] >= 2 && v[j] >= 24 {
                answer += 1;
            }
            if v[i] >= 4 && v[j] >= 14 {
                answer += 1;
            }
            for k in (1..=n).filter(|&k| k != i && k != j) {
                if v[i] >= 2 && v[j] >= 4 && v[k] >= 4 && j < k {
                    answer += 1;
                }
            }
        }
    }
    println!("{}", answer);
}
