use std::mem;
use std::time::Instant;

pub fn sieve_of_eratosthenes(limit: usize) -> Vec<usize> {
    if limit < 2 {
        return vec![];
    }

    let mut is_prime = vec![true; limit + 1];
    is_prime[0] = false;
    is_prime[1] = false;

    let sqrt_limit = (limit as f64).sqrt() as usize;
    for i in 2..=sqrt_limit {
        if is_prime[i] {
            let mut j = i * i;
            while j <= limit {
                is_prime[j] = false;
                j += i;
            }
        }
    }

    is_prime
        .iter()
        .enumerate()
        .filter(|(_, &is_prime)| is_prime)
        .map(|(num, _)| num)
        .collect()
}

fn main() {
    let limits = vec![100, 1000, 10000, 100000, 1000000];

    for limit in limits {
        println!("\nTesting with limit: {}", limit);

        // Measure time
        let start = Instant::now();
        let primes = sieve_of_eratosthenes(limit);
        let duration = start.elapsed();

        // Measure memory
        let memory = mem::size_of_val(&primes) + mem::size_of_val(&*primes);

        println!("Time taken: {:?}", duration);
        println!("Memory used: {} bytes", memory);
        println!("Number of primes found: {}", primes.len());

        if limit == 100 {
            println!("First few primes: {:?}", &primes[..10]);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_numbers() {
        assert_eq!(sieve_of_eratosthenes(1), vec![]);
        assert_eq!(sieve_of_eratosthenes(2), vec![2]);
        assert_eq!(sieve_of_eratosthenes(3), vec![2, 3]);
    }

    #[test]
    fn test_first_twenty() {
        let expected = vec![2, 3, 5, 7, 11, 13, 17, 19];
        assert_eq!(sieve_of_eratosthenes(20), expected);
    }

    #[test]
    fn test_prime_count() {
        assert_eq!(sieve_of_eratosthenes(100).len(), 25); // There are 25 primes below 100
        assert_eq!(sieve_of_eratosthenes(1000).len(), 168); // There are 168 primes below 1000
    }
}

