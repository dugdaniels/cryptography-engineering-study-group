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

## Ch 4:

Q1. Let `P` be a plaintext and let `ℓ(P)` be the length of `P` in bytes. Let `b` be the block size of the block cipher in bytes. Explain why the following is not a good padding scheme:

* Determine the minimum number of padding bytes necessary in order to pad the plaintext to a block boundary. This is a number `n` which satisfies `0 ≤ n ≤ b − 1` and `n + ℓ(P)` is a multiple of `b`. Pad the plaintext with `n` bytes of value `n`.

> This is not a good padding scheme because it is not possible to distinguish between a plaintext that is already a multiple of the block size and a plaintext that has been padded. In practice all padding rules should add a minimum of one byte. 

Q3. Suppose you, as an attacker, observe the following 32-byte ciphertext `C` (in hex) `46 64 DC 06 97 BB FE 69 33 07 15 07 9B A6 C2 3D 2B 84 DE 4F 90 8D 7D 34 AA CE 96 8B 64 F3 DF 75` and the following 32-byte ciphertext `C′` (also in hex) `51 7E CC 05 C3 BD EA 3B 33 57 0E 1B D8 97 D5 30 7B D0 91 6B 8D 82 6B 35 B7 8B BB 8D 74 E2 C7 3B`.

Suppose you know these ciphertexts were generated using CTR mode with the same nonce. The nonce is implicit, so it is not included in the ciphertext. You also know that the plaintext `P` corresponding to `C` is `43 72 79 70 74 6F 67 72 61 70 68 79 20 43 72 79 70 74 6F 67 72 61 70 68 79 20 43 72 79 70 74 6F`.

What information, if any, can you infer about the plaintext `P′` corresponding to `C′`?

> Because the plaintext corresponding to the ciphertext of one message is known, the keystream can be derived (`ciphertext = plaintext ^ keystream`), and you can decrypt any other message encrypted with the same keystream.

Q4. The ciphertext (in hex): `87 F3 48 FF 79 B8 11 AF 38 57 D6 71 8E 5F 0F 91 7C 3D 26 F7 73 77 63 5A 5E 43 E9 B5 CC 5D 05 92 6E 26 FF C5 22 0D C7 D4 05 F1 70 86 70 E6 E0 17` was generated with the 256-bit AES key (also in hex) `80 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 01` using CBC mode with a random IV. The IV is included at the beginning of the ciphertext. Decrypt this ciphertext. You may use an existing cryptography library for this exercise.

> Completed in `session-two/src/aes_cbc.rs`

Q6; Let `P1, P2` be a message that is two blocks long, and let `P′1` be a message that is one block long. Let `C0, C1, C2` be the encryption of `P1, P2` using CBC mode with a random IV and a random key, and let `C′0, C′1` be the encryption of `P′1` using CBC mode with a random IV and the same key. Suppose an attacker knows `P1, P2` and suppose the attacker intercepted and thus know `C0, C1, C2` and `C′0, C′1`. Further suppose that, by random chance, `C1 = C2`. Show that the attacker can compute `P′1`.

> ```
> E(K, P2 ^ C1) == E(K, P′1 ^ C′O)  // from CBC specifications
> P2 ^ C1 == P′1 ^ C′O              // decrypt both sides
> P2 ^ P′1 == C1 ^ C′O              // basic algebra
> ```

## General

Implement a pair of functions: A PKCS message padding function, and a padding validation function that takes a message and validates whether it has a correct padding.

> Completed in `session-two/src/pkcs.rs`