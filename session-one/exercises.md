# Exercises

## Ch 1:

Q10. Describe a concrete example where improving the security of a system against one type of attack can increase the likelihood of other attacks.

> Often, with residential houses, there is a window on or near the front door. Using a conventional lock on the door makes the door more secure, but the window becomes weakest link in the system. Someone can simply break the window to unlock the door.

## Ch 2:

Q3. Consider a group of 30 people who wish to establish pair-wise secure communications using symmetric-key cryptography. How many keys need to be exchanged in total.

> 435

Q4. Suppose Bob receives a messages signed using a digital signature scheme with Alice’s secret signing key. Does it prove that Alice saw the message and chose to sign.

> No, it only proves that the message signature was computed using her secret key.

Q6. Suppose a chosen-ciphertext attacker cannot recover the secret decryption key for an encryption scheme. Does this mean the encryption scheme is secure?

> No, there are attack models that only use the plaintexts and/or cyphertexts to break the encryption scheme.

Q7. Consider a symmetric-key cryptosystem in which cryptographic keys are randomly selected from the set of all n-bit strings. Approximately what should n be in order to provide 128 bits of security against a birthday attack.

> ```rust
> 2.pow(256)
> ```

## General

Suppose you read about RSA encryption and wanted to find it’s standard specification. Where would you look?

> I would first get a high level overview of the algorithm from Wikipedia. Then, I would look for the original paper that introduced it. 

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

> Completed in `session-one/src/lib.rs`

What is a side channel attack? Is your cipher implementation constant time?

> A side channel attack is an attack that is based on extra information that is available as a result of the way it is implemented, rather than a flaw in the algorithm itself. My cipher implementation is not constant time.

Extras:
* Read New Directions in Cryptography.
* Consider ways to contribute what you learned this week to the Uncloak knowledge graph.
