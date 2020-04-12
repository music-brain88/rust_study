fn main() {
    println!("FizzBuzz game");
    for i in 1..10001 {
       let x: String; 
       println!("{}", match i%15 {
            0 => "FizzBuzz",
            3|6|9|12 => "Fizz",
            5|10 => "Buzz",
            _ => {x=i.to_string(); &x},
       });
    }
}
