use std::io;

fn main() {
    loop {
        let mut fib_input = String::new();
        println!("Input digit of Fibonacci sequence to calculate:");
        io::stdin()
            .read_line(&mut fib_input)
            .expect("Failed to read line");
        let fib_input: u128 = match fib_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Not a number");
                continue;
            }
        };
        let fib_num = fibo(fib_input);
        println!(
            "{} is number {} of the Fibonacci sequence",
            fib_num, fib_input
        );
        break;
    }
}

fn fibo(n: u128) -> u128 {
    let mut x: u128 = 1;
    let mut y: u128;
    let mut prev: u128 = 0;
    let mut counter: u128 = 1;
    while counter < n {
        y = x + prev;
        // println!("y = {} and x = {}", y, x);
        prev = x;
        x = y;
        // println!("y = {} and x = {} and prev = {}", y, x, prev);
        counter += 1;
        // println!("{} is current num", x);
    }
    x
}
