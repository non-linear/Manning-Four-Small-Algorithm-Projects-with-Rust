use std::io::{self, Write};

fn main() {
    use std::time::Instant;

    let num = get_i64("number: ");

    println!("Find the factors the slow way.");
    let start1 = Instant::now();
    let mut factors1 = find_factors(num);
    let duration1 = start1.elapsed();
    println!("find_factors: {:?} seconds", duration1);
    print_numbers(&mut factors1);
    println!("Product: {}", multiply_vector(factors1));
    println!();

    println!("Use the Euler's sieve to find the factors.");
    let start2 = Instant::now();
    let primes = sieve_to_primes(sieve_of_eratosthenes(1_000_000_000));
    let mut factors2 = find_factors_sieve(primes, num);
    let duration2 = start2.elapsed();
    println!("find_factors_sieve: {:?} seconds", duration2);
    print_numbers(&mut factors2);
    println!("Product: {}", multiply_vector(factors2));
    println!();
}

fn find_factors(mut num: i64) -> Vec<i64> {
    let mut factors = Vec::<i64>::new();
    while num % 2 == 0 {
        num /= 2;
        factors.push(2);
    }
    // Take out other primes.
    let mut factor = 3;
    while factor * factor <= num {
        if num % factor == 0 {
            factors.push(factor);
            num /= factor;
        } else {
            factor += 2;
        }
    }
    if num > 1 {
        factors.push(num);
    }
    return factors;
}

fn find_factors_sieve(primes: Vec<i64>, mut num: i64) -> Vec<i64> {
    let mut factors = Vec::<i64>::new();
    let mut prime_idx: usize = 1;
    let mut prime = primes[prime_idx];

    while num > 1 && prime <= num && prime_idx < primes.len() - 1 {
        while num % prime == 0 {
            num /= prime;
            factors.push(prime);
        }
        prime_idx += 1;
        prime = primes[prime_idx];
    }
    return factors;
}

// Print the vector of numbers.
fn print_numbers(primes: &mut Vec<i64>) {
    for prime in primes {
        print!("{prime} ");
    }
    println!();
}

// Build a sieve of Eratosthenes.
fn sieve_of_eratosthenes(max: usize) -> Vec<bool> {
    let mut sieve: Vec<bool> = vec![true; max];
    for factor in 3..max {
        if !sieve[factor] {
            continue;
        }
        for multiple in 2..max {
            let candidate = factor * multiple;

            if candidate >= max {
                break;
            }
            sieve[candidate] = false;
        }
    }
    return sieve;
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

fn multiply_vector(vec: Vec<i64>) -> i64 {
    return vec.iter().fold(1, |acc, x| acc * x);
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
