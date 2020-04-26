use std::io;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).ok();
    let sample: Vec<i32> = s.trim().split_whitespace().map(|e| e.parse().ok().unwrap()).collect();

    println!("sample: {:?}", sample);
}
