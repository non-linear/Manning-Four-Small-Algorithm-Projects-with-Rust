use utilities::get_i64::get_i64;

fn fibonacci_on_the_fly(values: &mut Vec<i64>, n: i64) -> i64 {
    let m = n as usize;
    if values.len() <= m {
        let fib_i_m1 = fibonacci_on_the_fly(values, n - 1);
        let fib_i_m2 = fibonacci_on_the_fly(values, n - 2);
        values.push(fib_i_m1 + fib_i_m2);
    }
    return values[m];
}
fn main() {
    // Create a vector for fill-on-the-fly.
    let mut fill_on_the_fly_values: Vec<i64> = vec![0, 1];

    loop {
        // Prompt the user for n.
        let n = get_i64("N: ");

        // Calculate the Fibonacci number.
        println!(
            "On the fly: {}",
            fibonacci_on_the_fly(&mut fill_on_the_fly_values, n)
        );
        println!("Bottom up:  {}", fibonacci_bottom_up(n));
        println!();
    }
}

fn fibonacci_bottom_up(n: i64) -> i64 {
    if n <= 1 {
        return n;
    }

    let mut fib_i_minus_2 = 0i64;
    let mut fib_i_minus_1 = 1i64;
    let mut fib_i = fib_i_minus_1 + fib_i_minus_2;
    for _i in 1i64..n {
        // Calculate this value of fib_i.
        fib_i = fib_i_minus_1 + fib_i_minus_2;

        // Set fib_i_minus_2 and fib_i_minus_1 for the next value.
        fib_i_minus_2 = fib_i_minus_1;
        fib_i_minus_1 = fib_i;
    }
    return fib_i;
}
