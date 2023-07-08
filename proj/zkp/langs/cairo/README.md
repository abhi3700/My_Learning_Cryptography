# Cairo

Follow my repo for playground: [Cairo Playground](https://github.com/abhi3700/cairo-playground)

## Overview

- A language for writing zk-STARKs maths circuits.
- Analogous to Circom language (for zkSNARK).
- Previously, Cairo was built on top of Python.
- Now, Cairo is the Rust-inspired, native smart contract language for Starknet.
- Cairo is the first Turing-complete language for creating STARK-provable programs for general computation.

### Cairo vs Solidity

| Context                       | Cairo                                                                                                                                 | Solidity                                                                                |
| ----------------------------- | ------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------- |
| SC language type              | Turing complete                                                                                                                       | Turing complete                                                                         |
| SC language                   | Cairo                                                                                                                                 | Solidity                                                                                |
| VM                            | CVM                                                                                                                                   | EVM                                                                                     |
| SC binary                     | CASM                                                                                                                                  | EVM bytecode                                                                            |
| Network                       | Starknet                                                                                                                              | EVM L1 chains                                                                           |
| Write provable programs using | Cairo                                                                                                                                 | ❌                                                                                      |
| Network load                  | **Low**: Because of the proof generation once by the L2 node alone (unlike SNARKs) & then multiple times verification by the L1 nodes | **High**: Each node has to validate the info (includes generating proof & verifying it) |
| Inheritance & polymorphism    | Not directly, but importing modules, specific functions, traits possible                                                              | inheritance, polymorphism possible                                                      |

> **Network load (scalability)**: Here, the verification time is to be minimal considering multiple times verification. Even if the prover time is high, it's fine as done only once.

### CVM vs EVM

| Context          | CVM                                                                                                                                                                    | EVM                              |
| ---------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------- |
| **Memory model** | single-write-only memory model so that the value is retained until the execution of the program is complete, as they rely on verifying the correctness of computations | multiple read-write memory model |

## Installation

### [Cairo [Rust]](https://github.com/abhi3700/cairo-playground/README.md#installation)

More can be found in my [cairo-playground](https://github.com/abhi3700/cairo-playground).

### Python

#### CLI

```bash
brew install gmp
pip3 install ecdsa fastecdsa sympy
pip3 install cairo-lang
```

#### Editor

VSCode Extensions:

- [Cairo language support for StarkNet](https://marketplace.visualstudio.com/items?itemName=ericglau.cairo-ls) [Python]

> Cairo was tested with python3.9

## References

- [Starknet Book](https://book.starknet.io/)
- [Learn Smart contracts using cairo](https://book.starknet.io/chapter_2/index.html)
- [The Cairo Programming Language](https://cairo-book.github.io/title-page.html)
  > Very detailed, especially in terms of theory.
- [Official Language guide](https://www.cairo-lang.org/)
- [Cairo Playground](https://www.cairo-lang.org/playground/)
- [Learn Stark sequencers](https://book.starknet.io/chapter_8/sequencers.html)
- [Scarb toolchain documentation](https://docs.swmansion.com/scarb/docs)

### Videos

- YT channels
  - [StarkWare](https://www.youtube.com/channel/UCnDWguR8mE2oDBsjhQkgbvg/playlists)
  - [StarkNet Africa](https://www.youtube.com/@starknetafrica)
- [Starknet Workshops](https://www.youtube.com/playlist?list=PLcIyXLwiPilV5RBZj43AX1FY4FJMWHFTY)
- [Cairo Bootcamp 1.0 | 05-Dec-2022](https://www.youtube.com/playlist?list=PLKhUlfTgU76DVMLsoGD8C30pCWh66peRC)
- [STARK Devs Series | 19-Jan-2023](https://www.youtube.com/playlist?list=PLKhUlfTgU76CwprjKSBJw25sTuiIBhVvc)
- [StarkNet ByteSized YT playlist](https://www.youtube.com/playlist?list=PLcIyXLwiPilVfjUeZ-XfD9I097ksXjKyU)
  > very short length videos 2-3 mins each.
