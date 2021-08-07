use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String,
        mut k: usize,
    }
    let mut v = s.bytes().collect::<Vec<_>>();
    for u in v.iter_mut() {
        if *u == b'a' {
            continue;
        }
        let c = (1 + b'z' - *u) as usize;
        if c <= k {
            *u = b'a';
            k -= c;
        }
    }
    if let Some(last) = v.last_mut() {
        *last = b'a' + (*last - b'a' + ((k % 26) as u8)) % 26;
    }
    println!("{}", v.iter().map(|u| *u as char).collect::<String>());
}
