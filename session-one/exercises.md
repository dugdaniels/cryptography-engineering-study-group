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

# General

// TODO: Add general questions