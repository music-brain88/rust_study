fn main() {
    println!("FizzBuzz game");
    for i in 1..101 {
        if i % 3 == 0 && i % 5 == 0 {
            println!("{} is FizzBuzz", i);
        } else if i%3 == 0 {
            println!("{} is Fizz", i);
        } else if i%5 == 0 {
            println!("{} is Buzz", i);
        } else {
            println!("{}", i)
        }
    }
}
