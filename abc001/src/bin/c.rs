use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        deg: usize, dis: u32,
    }
    let dir = [
        "N", "NNE", "NE", "ENE", "E", "ESE", "SE", "SSE", "S", "SSW", "SW", "WSW", "W", "WNW",
        "NW", "NNW",
    ][((deg * 10 + 1125) / 2250) % 16];
    let w = match (dis as f32 / 6.0).round() as u32 {
        0..=2 => 0,
        3..=15 => 1,
        16..=33 => 2,
        34..=54 => 3,
        55..=79 => 4,
        80..=107 => 5,
        108..=138 => 6,
        139..=171 => 7,
        172..=207 => 8,
        208..=244 => 9,
        245..=284 => 10,
        285..=326 => 11,
        _ => 12,
    };
    println!("{} {}", if w == 0 { "C" } else { dir }, w);
}
