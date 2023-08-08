# StarkNet

## About

- StarkNet is a zk-STARKs based layer-2 scaling solution for Ethereum.
- `Cairo` is the first Turing-complete language for creating provable programs for general computation. Previously based on Python, now based on Rust.
- It enables us to write smart contracts in cairo, called StarkNet contracts. E.g. [ERC20 token](https://github.com/starkware-libs/cairo/blob/d485f5ffd0c444d900cdcac57b9e745dcc280fba/crates/cairo-lang-starknet/test_data/erc20.cairo).
- `Cairo` based on rust (previously python). [crate](https://github.com/starkware-libs/cairo/tree/d485f5ffd0c444d900cdcac57b9e745dcc280fba/crates/cairo-lang-starknet).
- STARK proof is called **AIR (Algebraic Intermediate Representation)** which is the succicnt representation of the computation.
- The computational language is **Cairo** in case of Starknet.
- The computational language is **Rust** in case of [Winterfell](./winterfell.md).

## Installation

> For macOS M1.

refer [this](../../langs/cairo/README.md#installation) for cairo [`Rust (v1.x)`],[`Python (v0.x)`].

## Diagrams

Refer [this](./starknet.drawio).

## References

- [Understanding Zero-Knowledge Proofs in 15 Mins through SNARK and STARK](https://intelchen.medium.com/understanding-zero-knowledge-proofs-in-15-mins-through-snark-and-stark-7638311f0cc9) 🧑🏻‍💻
