use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        h: u64, w: u64,
    }
    if h * w % 3 == 0 {
        println!("0");
        return;
    }
    let s = |h: u64, w: u64| {
        let mut v = Vec::new();
        v.push((h + 1) / 3 * w);
        let h = h - (h + 1) / 3;
        if h * w % 2 == 0 {
            v.push(h * w / 2);
            v.push(h * w / 2);
        } else if h > w {
            v.push(h / 2 * w);
            v.push((h - h / 2) * w);
        } else {
            v.push(w / 2 * h);
            v.push((w - w / 2) * h);
        }
        v.sort_unstable();
        v[2] - v[0]
    };
    println!("{}", s(h, w).min(s(w, h)));
}
