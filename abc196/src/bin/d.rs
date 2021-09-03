use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        h: usize, w: usize, a: usize, _b: usize,
    }
    let mut answer = 0;
    backtrack(a, w, &mut vec![false; h * w], 0, &mut answer);
    println!("{}", answer)
}

fn backtrack(a: usize, w: usize, v: &mut Vec<bool>, p: usize, answer: &mut u32) {
    if a == 0 {
        *answer += 1;
        return;
    }
    for i in p..v.len() {
        if v[i] {
            continue;
        }
        let mut js = vec![1, w];
        js.dedup();
        for &j in &js {
            if i + j < v.len() && !v[i + j] && (j == w || i / w == (i + 1) / w) {
                v[i] = true;
                v[i + j] = true;
                backtrack(a - 1, w, v, i + 1, answer);
                v[i] = false;
                v[i + j] = false;
            }
        }
    }
}
