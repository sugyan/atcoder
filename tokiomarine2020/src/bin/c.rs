use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize, k: usize,
        mut a: [usize; n],
    }
    for _ in 0..k {
        let mut v = vec![0; n];
        for (i, &a) in a.iter().enumerate() {
            v[i.max(a) - a] += 1;
            if i + a + 1 < n {
                v[i + a + 1] -= 1;
            }
        }
        a = v
            .iter()
            .scan(0, |state, &x| {
                *state = n.min((*state as i32 + x) as usize);
                Some(*state)
            })
            .collect();
        if a.iter().all(|&a| a == n) {
            break;
        }
    }
    println!(
        "{}",
        a.into_iter()
            .map(|i| i.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
