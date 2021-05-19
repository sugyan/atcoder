use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        mut n: usize, k: usize,
        d: [usize; k],
    }
    let v = d.iter().fold([true; 10], |mut acc, &d| {
        acc[d] = false;
        acc
    });
    loop {
        if n.to_string()
            .as_bytes()
            .iter()
            .all(|&d| v[(d - b'0') as usize])
        {
            break;
        }
        n += 1;
    }
    println!("{}", n);
}
