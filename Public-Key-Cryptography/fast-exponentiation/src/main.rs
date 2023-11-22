use std::io::{self, Write};
fn main() {
    loop {
        // Prompt the user for n.
        let num = get_i64("number: ");
        let pow = get_i64("exponent: ");
        let modulus = get_i64("modulus: ");

        // Fast exponentiation
        print!(
            "{}^{} = {} (fast exponentiation)\n",
            num,
            pow,
            fast_exp(num, pow)
        );
        print!("{}^{} = {} (num.pow(…))\n", num, pow, num.pow(pow as u32));

        // Fast exponentiation with modulus
        print!(
            "{}^{} % {} = {} (fast exponentiation in modulus)\n",
            num,
            pow,
            modulus,
            fast_exp_mod(num, pow, modulus)
        );
        print!(
            "{}^{} % {} = {} (num.pow(…) % modulus)\n",
            num,
            pow,
            modulus,
            num.pow(pow as u32) % modulus
        );
    }
}

// Perform fast exponentiation.
fn fast_exp(mut num: i64, mut pow: i64) -> i64 {
    let mut result: i64 = 1;
    while pow > 0 {
        if pow % 2 == 1 {
            result *= num;
        }
        pow /= 2;
        num *= num;
    }
    return result;
}

// Perform fast exponentiation in a modulus.
#[allow(dead_code)]
fn fast_exp_mod(mut num: i64, mut pow: i64, modulus: i64) -> i64 {
    let mut result: i64 = 1;
    while pow > 0 {
        if pow % 2 == 1 {
            result = (result * num) % modulus;
        }
        pow /= 2;
        num = (num * num) % modulus;
    }
    return result;
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
