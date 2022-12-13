# Exercises

## Ch 5:

Q3. Consider SHA-512-n, a hash function that first runs SHA-512 and then outputs only the first `n` bits of the result. Write a program that uses a birthday attack to find and output a collision on SHA-512-n, where `n` is a multiple of 8 between 8 and 48. Your program may use an existing cryptography library. Time how long your program takes when `n` is 16, averaged over five runs for each `n`.

> Implemented in `session-three/src/sha512n.rs`. Benchmarked in `session-three/benches/sha512n_benchmark.rs`.

Q4. Let SHA-512-n be as in the previous exercise. Write a program that finds a message `M` that hashes to the following value under SHA-512-16 (in hex): `3D 4B`. How many tries would you expect the algorithm to need? Running the algorithm 5 times, How many tries did it take on average?

> Implemented in `session-three/src/sha512n.rs`.

Q5. With command line tools or Criterion, benchmark the blake3 hash (default is 256 bit output), and compare it to benches of SHA3-256 and SHA-256 (when written without a number, SHA is assumed to be SHA2).

> Completed in `session-three/benches/hash_benchmark.rs`.

## Ch 6:

Q5. Using an existing cryptography library, compute the MAC of the message: `4D 41 43 73 20 61 72 65 20 76 65 72 79 20 75 73 65 66 75 6C 20 69 6E 20 63 72 79 70 74 6F 67 72 61 70 68 79 21 20 20 20 20 20 20 20 20 20 20 20` using CBC-MAC with AES and the 256-bit key: `80 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 01`.

> Completed in `session-three/src/mac.rs`.