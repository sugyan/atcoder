use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut p: [u32; n],
    }
    let mut used = vec![false; n - 1];
    let mut answer = Vec::new();
    let mut i = 0;
    while i < n {
        let mut j = i;
        while j < n && p[j] != (i + 1) as u32 {
            j += 1;
        }
        for k in (i..j).rev() {
            if used[k] {
                println!("{}", -1);
                return;
            }
            used[k] = true;
            answer.push(k + 1);
            p.swap(k, k + 1);
        }
        i += 1;
    }
    if used.iter().all(|&b| b) {
        for &a in &answer {
            println!("{}", a);
        }
    } else {
        println!("{}", -1);
    }
}
