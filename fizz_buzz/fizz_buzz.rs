fn main() {
    println!("FizzBuzz game");
    for i in 1..101 {
        if i % 3 == 0 && i % 5 == 0 {
            println!("{} is FizzBuzz", i);
        }
        if i % 3 == 0 {
            println!("{} is Fizz", i);
        }
        if i % 5 == 0 {
            println!("{} is Buzz", i);
        }
    }
}
