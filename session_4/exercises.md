# Exercises

Justify or disqualify each of the following schemes, with message `m`, tag `t`, and ciphertext `c`.
* `t = MAC(m)` `c = E(m)`, send `(c,t)`
* `t = MAC(m)` `c = E(m||t)`, send `c`
* `c = E(m)` `t = MAC(c)`, send `(c,t)`

You're the adversary, watching a TLS handshake. Pick three steps from TLS Handshake - OSDev Wiki, and describe how the step prevents you from (pick one):
* reading message content (confidentiality)
* tampering with message content (integrity)
* impersonating either party (authenticity)