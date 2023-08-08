# Winterfell

## Overview

Write stark proofs using Winterfell in rust.

> There are other options like StarkWare's Cairo, but I anticipate Winterfell (from Facebook) is going to be pretty easy to use quickly.

**Components**:

- Prover
- VDF (Verifiable Delay Function)

## Installation

## Diagrams

Refer [this](./starknet.drawio).

## Concepts

- The **proof size** can be reduced by removing the feature (i.e. post-quantum security) for the proof.
- The **prover time** can be reduced by introducing MPC (Multi-Party Computation) where the computation is done in parallel by multiple parties.

---

![](../../../../img/winterfell_verifyable_delay.png)

![](../../../../img/winterfell_verifyable_delay_function.png)

## Coding

[Code example](../../libs/winterfell/demo)

## References

- [ZK HACK mini - Writing STARK proofs with Winterfell](https://www.youtube.com/watch?v=LBTrX0Ukdvs) ✅
