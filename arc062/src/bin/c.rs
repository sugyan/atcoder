use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut ta: [(u64, u64); n],
    }
    let mut m = (1, 1);
    for (t, a) in ta.iter_mut() {
        if *t < m.0 || *a < m.1 {
            let f = (m.0 / *t + if m.0 % *t == 0 { 0 } else { 1 })
                .max(m.1 / *a + if m.1 % *a == 0 { 0 } else { 1 });
            *t *= f;
            *a *= f;
        }
        m = (*t, *a);
    }
    println!("{}", m.0 + m.1);
}
