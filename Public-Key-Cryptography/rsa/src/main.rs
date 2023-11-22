use utilities::{get_i64::get_i64, prng::Prng};

fn main() {
    let mut prng = Prng::new();

    let p = find_prime(&mut prng, 1000, 10000, 2);
    let q = find_prime(&mut prng, 1000, 10000, 2);
    let n = p * q;
    let λ_n = totient(p, q);
    let e = random_exponent(&mut prng, λ_n);
    println!("4");
    let d = inverse_mod(e, λ_n);

    println!("*** Public ***");
    println!("Public key modulus:    {n}");
    println!("Public key exponent e: {e}");

    println!("*** Private ***");
    println!("Primes:    {p}, {q}");
    println!("λ(n):      {λ_n}");
    println!("d:         {d}");

    loop {
        let message = get_i64("# Message: ");
        if message < 1 {
            break;
        }
        let encrypted_message = fast_exp_mod(message, e, n);
        println!("encrypted: {encrypted_message}");

        let decrypted_message = fast_exp_mod(encrypted_message, d, n);
        println!("decrypted: {decrypted_message}");
    }
}

// Calculate Carmichael's totient function λ(n)
// where n = p * q and p and q are prime.
fn totient(p: i64, q: i64) -> i64 {
    return lcm(p - 1, q - 1);
}

fn lcm(a: i64, b: i64) -> i64 {
    return (b / gcd(a, b)) * a;
}

fn gcd(a: i64, b: i64) -> i64 {
    // final case
    if b == 0 {
        return a;
    }

    // recursive case
    gcd(b, a % b)
}

// Pick a random exponent e in the range [3, λ_n)
// such that gcd(e, λ_n) = 1.
fn random_exponent(prng: &mut Prng, λ_n: i64) -> i64 {
    loop {
        let e = prng.next_i64(3, λ_n);
        if gcd(e, λ_n) == 1 {
            return e;
        }
    }
}

fn inverse_mod(a: i64, n: i64) -> i64 {
    let mut t = 0;
    let mut new_t = 1;
    let mut r = n;
    let mut new_r = a;
    while new_r != 0 {
        let quotient = r / new_r;
        (t, new_t) = (new_t, t - quotient * new_t);
        (r, new_r) = (new_r, r - quotient * new_r);
    }
    if r > 1 {
        panic!("not invertible!");
    }
    if t < 0 {
        t += n;
    }
    return t;
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
        if is_probably_prime(prng, p, num_tests) {
            return p as i64;
        };
    }
}
