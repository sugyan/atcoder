use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [i32; n],
    }
    let (mut t, mut f) = (Vec::new(), Vec::new());
    for &a in &a {
        if a > 0 {
            t.push(a);
        } else {
            f.push(a);
        }
    }
    if t.is_empty() {
        f.sort_unstable();
        t.push(f.pop().unwrap());
    }
    if f.is_empty() {
        t.sort_unstable();
        t.reverse();
        f.push(t.pop().unwrap());
    }
    println!("{}", t.iter().sum::<i32>() - f.iter().sum::<i32>());
    let mut m = if t.len() > 1 {
        let mut m = f.pop().unwrap();
        while t.len() > 1 {
            let v = t.pop().unwrap();
            println!("{} {}", m, v);
            m -= v;
        }
        println!("{} {}", t[0], m);
        t[0] - m
    } else {
        t[0]
    };
    for &v in &f {
        println!("{} {}", m, v);
        m -= v;
    }
}
