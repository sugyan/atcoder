use proconio::marker::Bytes;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: Bytes,
        b: Bytes,
        c: Bytes,
    }
    let s = [a, b, c];
    let mut i = [0, 0, 0];
    let mut j = 0;
    loop {
        if i[j] == s[j].len() {
            println!("{}", (b'A' + j as u8) as char);
            break;
        }
        let n = s[j][i[j]];
        i[j] += 1;
        j = (n - b'a') as usize;
    }
}
