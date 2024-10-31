import unittest
from main import sieve_of_eratosthenes


class TestSieveOfEratosthenes(unittest.TestCase):
    def test_small_numbers(self):
        self.assertEqual(sieve_of_eratosthenes(1), [])
        self.assertEqual(sieve_of_eratosthenes(2), [2])
        self.assertEqual(sieve_of_eratosthenes(3), [2, 3])

    def test_first_twenty(self):
        expected = [2, 3, 5, 7, 11, 13, 17, 19]
        self.assertEqual(sieve_of_eratosthenes(20), expected)

    def test_prime_count(self):
        self.assertEqual(
            len(sieve_of_eratosthenes(100)), 25
        )  # There are 25 primes below 100
        self.assertEqual(
            len(sieve_of_eratosthenes(1000)), 168
        )  # There are 168 primes below 1000


if __name__ == "__main__":
    unittest.main()
