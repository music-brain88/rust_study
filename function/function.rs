
fn print_number(x: i32) {
    println!("x is {}", x); 
}

fn print_sum(x: i32, y:i32) {
    println!("sum is {}", x + y);
}

fn main() {
    print_number(5);

    print_sum(3, 5);
}
