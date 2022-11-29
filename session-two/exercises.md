# Exercises

## Ch 3:

Q1. How much space would be required to store a table for an entire idealized block cipher that operates on 64-bit blocks and has 80-bit keys?

> ```rust
> 2.pow(80) * 2.pow(64) * 64
> ```

1,427,247,692,705,959,881,058,285,969,449,495,136,380,000,000 bits
> ```

Q:5. Suppose you have a processor that can perform a single DES encryption or decryption operation in `2.pow(-26)` seconds. Suppose you also have a large number of plaintext-ciphertext pairs for DES under a single unknown key. How many hours would it take, on average, to find that DES key, using an exhaustive search approach and a single processor? How many hours would it take, with a collection of `2.pow(14)` processors?

> ```rust
> 2.pow(56-1) * 2.pow(-26) / 3_600 // 149,130.809 hours on single processor
> 2.pow(56-1) * 2.pow(-26) / 3_600 / 2.pow(14) // 9.102 hours on 2.pow(14) processors
> ```

Q6. Consider a new block cipher, DES2, that consists only of two rounds of the DES block cipher. DES2 has the same block and key size as DES. For this question you should consider the DES F function as a black box that takes two inputs, a 32-bit data segment and a 48-bit round key, and that produces a 32-bit output. Suppose you have a large number of plaintext-ciphertext pairs for DES2 under a single, unknown key. Given an algorithm for recovering the 48-bit round key for round 1 and the 48-bit round key for round 2. Your algorithm should require fewer operations than an exhaustive search for an entire 56-bit DES key. Can your algorithm be converted into a distinguishable attack against DES2?

> Yes, but only as a generic distinguisher attack.

Q8. Familiarize yourself with a cryptographic CLI tools. A popular open source package is OpenSSL. Using an existing cryptographic library, decrypt the following ciphertext (in hex) `53 9B 33 3B 39 70 6D 14 90 28 CF E1 D9 D4 A4 07`
with the following 256-bit key (also in hex): `80 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 01` using AES.

> Completed in `session-two/src/simple_aes.rs`

Q9. Using an existing cryptography library, encrypt the following plaintext (in hex) `29 6C 93 FD F4 99 AA EB 41 94 BA BC 2E 63 56 1D` with the following 256-bit key from problem 8, using AES: `80 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 01`. Then re-encrypt and decrypt it using a 3072-bit RSA key with GnuPG, or your choice of asymmetric crypto CLI.

> Completed in `session-two/src/lib.rs`

Q10. Write a program that experimentally demonstrates the complementation property for DES. This program should take as input a key `K` and a plaintext `P`and demonstrate that the DES complementation property holds for this key and plaintext. You may use an existing cryptography library for this exercise.

> Completed in `session-two/src/des_complementation.rs`