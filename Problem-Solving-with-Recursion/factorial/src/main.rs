fn factorial(n: i64) -> i64 {
    match n {
        0 => 1,
        n => n * factorial(n - 1),
    }
}

fn main() {
    for n in 0..22 {
        println!("{}! = {}", n, factorial(n));
    }
}
