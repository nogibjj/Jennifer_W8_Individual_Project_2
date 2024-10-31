import time
import sys
from typing import List


def sieve_of_eratosthenes(limit: int) -> List[int]:
    if limit < 2:
        return []

    # Initialize the sieve
    is_prime = [True] * (limit + 1)
    is_prime[0] = is_prime[1] = False

    # Mark non-prime numbers
    sqrt_limit = int(limit**0.5)
    for i in range(2, sqrt_limit + 1):
        if is_prime[i]:
            for j in range(i * i, limit + 1, i):
                is_prime[j] = False

    # Collect prime numbers
    return [num for num, prime in enumerate(is_prime) if prime]


def measure_performance():
    limits = [100, 1000, 10000, 100000, 1000000]

    for limit in limits:
        print(f"\nTesting with limit: {limit}")

        # Measure time
        start_time = time.time()
        primes = sieve_of_eratosthenes(limit)
        duration = time.time() - start_time

        # Measure memory (approximate)
        memory = sys.getsizeof(primes)

        print(f"Time taken: {duration:.6f} seconds")
        print(f"Memory used: {memory} bytes")
        print(f"Number of primes found: {len(primes)}")

        if limit == 100:
            print(f"First few primes: {primes[:10]}")


if __name__ == "__main__":
    measure_performance()
