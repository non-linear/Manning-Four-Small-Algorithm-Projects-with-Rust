use std::io::{self, Write};

fn main() {
    let max = get_i64("Max: ");
    let mut sieve = sieve_of_eratosthenes(max as usize);
    println!("Sieve of Eratosthenes:");
    print_sieve(&mut sieve);

    let mut primes = sieve_to_primes(sieve);
    println!("Prime numbers:");
    print_numbers(&mut primes);
}
// Build a sieve of Eratosthenes.
fn sieve_of_eratosthenes(max: usize) -> Vec<bool> {
    let mut sieve: Vec<bool> = vec![true; max];
    for candidate in 4..max {
        if sieve[candidate] {
            for test_divisor in 2..max {
                if candidate % test_divisor == 0 && candidate / test_divisor > 1 {
                    sieve[candidate] = false;
                    break;
                }
            }
        }
    }
    return sieve;
}

// Print out the primes in the sieve.
fn print_sieve(sieve: &mut Vec<bool>) {
    for candidate in 0..sieve.len() {
        if sieve[candidate] {
            print!("{} ", candidate);
        } else {
            print!("X ");
        }
    }
    println!("");
}

// Convert the sieve into a vector holding prime numbers.
fn sieve_to_primes(sieve: Vec<bool>) -> Vec<i64> {
    let mut primes: Vec<i64> =
        Vec::with_capacity(
            sieve
                .iter()
                .fold(0, |acc, x| if *x { acc + 1 } else { acc }),
        );

    for candidate in 1..sieve.len() {
        if sieve[candidate] {
            primes.push(candidate as i64);
        }
    }
    return primes;
}

// Print the vector of numbers.
fn print_numbers(primes: &mut Vec<i64>) {
    for prime in primes {
        print!("{prime} ");
    }
    println!();
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
