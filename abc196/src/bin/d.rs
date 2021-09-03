use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        h: usize, w: usize, a: usize, _b: usize,
    }
    println!("{}", backtrack(&mut vec![false; h * w], w, a, 0));
}

fn backtrack(v: &mut Vec<bool>, w: usize, a: usize, p: usize) -> i32 {
    if a == 0 {
        return 1;
    }
    let mut ret = 0;
    for i in p..v.len() {
        if v[i] {
            continue;
        }
        for j in if w == 1 { vec![1] } else { vec![1, w] } {
            if i + j < v.len() && !v[i + j] && (j == w || i / w == (i + 1) / w) {
                v[i + j] = true;
                ret += backtrack(v, w, a - 1, i + 1);
                v[i + j] = false;
            }
        }
    }
    ret
}
