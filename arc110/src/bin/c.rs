use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut p: [u32; n],
    }
    let mut used = vec![false; n - 1];
    let mut answer = Vec::new();
    'swap: for i in 0..n {
        let mut j = i;
        while j < n && p[j] != (i + 1) as u32 {
            j += 1;
        }
        for k in (i..j).rev() {
            if used[k] {
                answer.clear();
                break 'swap;
            }
            used[k] = true;
            answer.push(k + 1);
            p.swap(k, k + 1);
        }
    }
    if answer.len() == n - 1 {
        for &a in &answer {
            println!("{}", a);
        }
    } else {
        println!("{}", -1);
    }
}
