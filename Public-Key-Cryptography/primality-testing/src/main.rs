use utilities::{get_i64::get_i64, prng::Prng};

const NUM_TESTS: i64 = 20;

fn main() {
    // Prepare a Prng.
    let mut prng = Prng::new();

    // Display the probability that a number is prime
    // if it passes all NUM_TESTS tests.
    let probability = 1.0 - 0.5_f32.powi(NUM_TESTS as i32);
    println!("Probability: {}%\n", probability);

    // Generate random primes.
    loop {
        // Get the number of digits.
        let num_digits = get_i64("# Digits (max 9): ");
        if num_digits < 1 {
            break;
        }

        // Calculate minimum and maximum values.
        let mut min = 10i64.pow((num_digits - 1) as u32);
        let max = 10 * min;
        if min == 1 {
            min = 2;
        } // 1 is not prime.

        // Find a prime.
        println!("Prime: {}", find_prime(&mut prng, min, max, NUM_TESTS));
    }
}

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

fn is_probably_prime(prng: &mut Prng, p: i64, num_tests: i64) -> bool {
    // Perform the tests.
    for _i in 0..num_tests {
        let n = prng.next_i64(2, p);

        if fast_exp_mod(n, p - 1, p) != 1 {
            return false;
        }
    }
    // If we survived all the tests, p is probably prime.
    return true;
}

// Probabilistically find a prime number within the range [min, max].
fn find_prime(prng: &mut Prng, min: i64, max: i64, num_tests: i64) -> i64 {
    // Try random numbers until we find a prime.
    loop {
        // Pick a random odd p.
        let p = prng.next_i64(min, max + 1);
        if p % 2 == 0 {
            continue;
        }
        // See if it's prime.
        println!("Trying {p}");
        if is_probably_prime(prng, p, num_tests) {
            return p as i64;
        };
    }
}
