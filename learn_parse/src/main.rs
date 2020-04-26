use std::io;

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn main() {

    let input1: i32 = read();    
    let input2: i32 = read();    

    let string: String = read();

    let answer = input1 + input2;

    print!("{} {}", answer, string);
}






