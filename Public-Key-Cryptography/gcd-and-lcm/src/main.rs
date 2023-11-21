use std::io::{self, Write};
fn main() {
    println!("Enter -1 to exit\n");
    loop {
        // Prompt the user for n.
        let a = get_i64("a: ");
        if a < 0 {
            break;
        }

        let b = get_i64("b: ");
        if b < 0 {
            break;
        }
        // Calculate the GCD
        print!("gcd({},{}) = {}\n", a, b, gcd(a, b));
        // Calculate the LCM
        print!("lcm({},{}) = {}\n", a, b, lcm(a, b));
    }
}

fn gcd(mut a: i64, mut b: i64) -> i64 {
    // data cleasing: both numbers shall be positive
    if a < 0 {
        a *= -1;
    }
    if b < 0 {
        b *= -1;
    }
    // a shall be larger than b (save one step)
    if a < b {
        return gcd(b, a);
    }
    // final cases
    if a == b {
        return a;
    }
    if a == 0 {
        return b;
    }
    if b == 0 {
        return a;
    }
    // recursive case
    gcd(b, a % b)
}

fn lcm(a: i64, b: i64) -> i64 {
    (b / gcd(a, b)) * a
}

// Prompt the user for an i64.
fn get_i64(prompt: &str) -> i64 {
    print!("{prompt}");
    io::stdout().flush().unwrap();

    let mut str_value = String::new();
    io::stdin()
        .read_line(&mut str_value)
        .expect("Error reading input");

    let trimmed = str_value.trim();
    return trimmed.parse::<i64>().expect("Error parsing integer");
}
