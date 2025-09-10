# 🔐 Cryptopals

Solutions to the **Matasano Cryptopals Cryptography Challenges**. Find the challenges [here](https://www.cryptopals.com/).

This repository is primarily implemented in **Rust**, with supporting utilities in [`challenge_utils`](./challenge_utils) for benchmarking and enabling multi-language experimentation.

*This project is a personal journey through the Cryptopals challenges — expect a mix of polished Rust implementations and exploratory code in other languages.*

---
## 📊 Cryptopals Progress by Set
| Set | Title                         | Challenges | Status    |
| --- | ----------------------------- | ---------- | --------- |
| 1   | Basics (XOR, ECB, hex/base64) | 1–8        | ✅ 8 / 8 |
| 2   | Block crypto                  | 9–16       | ⬜ 0 / 8 |
| 3   | Block & stream crypto         | 17–24      | ⬜ 0 / 8 |
| 4   | Stream crypto attacks         | 25–32      | ⬜ 0 / 8 |
| 5   | Diffie-Hellman & friends      | 33–40      | ⬜ 0 / 8 |
| 6   | RSA & DSA                     | 41–48      | ⬜ 0 / 8 |
| 7   | Hashes & MACs                 | 49–56      | ⬜ 0 / 8 |
| 8   | Attacks on ECDSA, SRP, etc.   | 57–66      | ⬜ 0 / 10 |

---
## 📂 Structure

* Each **set** of challenges lives in its own folder (e.g. `set_01`, `set_02`, …).
* Each **challenge** is an individual file called by its challenge NO(e.g. `cryptopals_01.rs`, `cryptopals_02.rs`).

## 🚀 Running

* Run a **single challenge**:

  ```bash
  cargo run --bin cryptopals_<NO>
  ```

* Run a **full set** of challenges:

  ```bash
  cargo run -p set_<NO>
  ```

---
