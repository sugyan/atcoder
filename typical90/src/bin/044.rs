use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize, q: usize,
        mut a: [u32; n],
        txy: [(u8, usize, usize); q],
    }
    let mut offset = 0;
    for &(t, x, y) in &txy {
        if t == 1 {
            a.swap((x + offset - 1) % n, (y + offset - 1) % n);
        }
        if t == 2 {
            offset = (offset + n - 1) % n;
        }
        if t == 3 {
            println!("{}", a[(x + offset - 1) % n]);
        }
    }
}
