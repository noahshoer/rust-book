fn main() {
    let count = 10;
    println!("Fibonacci number {count}: {}", fib(count));
    println!("Converted to celsius: {}", f_to_c(fib(count) as f32));
}

fn fib(x: i32) -> i32 {
    if x < 3 {
        1
    } else {
        fib(x - 1) + fib(x - 2)
    }
}

fn f_to_c(x: f32) -> f32 {
    (x - 32.0) / 1.8
}