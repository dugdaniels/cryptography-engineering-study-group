# Exercises

## Ch 1:

Q10. Describe a concrete example where improving the security of a system against one type of attack can increase the likelihood of other attacks.

> Using the latest and greatest encryption algorithm can seem like a security improvement, but it could also open you up to a yet unknown attack vector. It can take years for a new algorithm to be vetted and proven secure.

## Ch 2:

Q3. Consider a group of 30 people who wish to establish pair-wise secure communications using symmetric-key cryptography. How many keys need to be exchanged in total.

> 435

Q4. Suppose Bob receives a messages signed using a digital signature scheme with Alice’s secret signing key. Does it prove that Alice saw the message and chose to sign.

> No, it only proves that the message signature was computed using her secret key.

Q6. Suppose a chosen-ciphertext attacker cannot recover the secret decryption key for an encryption scheme. Does this mean the encryption scheme is secure?

> No, the attacker may still be able to recover information about the message without knowing the key.

Q7. Consider a symmetric-key cryptosystem in which cryptographic keys are randomly selected from the set of all n-bit strings. Approximately what should n be in order to provide 128 bits of security against a birthday attack.

> 256

## General

Find two libraries for each of RSA, TLS/SSL, and AEAD. Evaluate the maturity each library, and skim the code. What about the library structure makes sense? How is their documentation? These links may help:
* https://cryptography.rs/
* https://lib.rs/ (librs is equivalent to crates.io, with a different interface)

> 👍

Benchmark the speed of an algorithm in the two different implementations with Criterion.
* User guide: https://bheisler.github.io/criterion.rs/book/index.html
* Video intro: https://youtu.be/eIB3Pd5LBkc

> Completed in `session-one/benches/rsa_benchmark.rs`

You want to understand a paper on a new polynomial commitment scheme, but you’ve been trying for more than an hour, and the math is over your head. What do you do?

> Ask for help in the Uncloak Discord server.

Implement the Vignère cipher in 100 lines or less.

> Completed in `session-one/src/vigenere.rs`

What is a side channel attack? Is your cipher implementation constant time?

> A side channel attack is an attack that is based on extra information that is available as a result of the way it is implemented, rather than a flaw in the algorithm itself. My cipher implementation is not constant time.

Extras:
* Read New Directions in Cryptography.
* Consider ways to contribute what you learned this week to the Uncloak knowledge graph.
