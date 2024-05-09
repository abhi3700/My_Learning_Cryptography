# Noir

1. `$ nargo new hello`
2. `$ cd hello`
3. `$ nargo check`: Checks the constraint system for errors. Also generates Prover, Verifier `.toml` files with empty values like this:

   ```toml
   # Prover.toml
   x = ""
   y = ""

   # Verifier.toml
   y = ""
   ```

4. Now, fill `Prover.toml` file with the values of `x` and `y` like this:

   ```toml
   # Prover.toml
   x = "1"
   y = "2"
   ```

5. `$ nargo prove`: Generates proof file(s) in `proofs/` directory and the Verifier.toml file is updated with 32-bytes hex string.

   > Requires internet connection to download the SRS file.
   >
   > The proof file content changes every time you run the command.

   ```sh
   ❯ nargo prove

   Downloading the Ignite SRS (1.1 KB)
   Downloaded the SRS successfully!

   Downloading the Ignite SRS (128 B)
   Downloaded the SRS successfully!
   ```

6. `$ nargo verify` to verify the statement. Nothing appears if everything is ok.
7. `$ nargo execute` to execute the statement.

   ```sh
   ❯ nargo execute
   [hello] Circuit witness successfully solved
   ```

8. `$ nargo test`: Run the tests for this program and check if the results are as expected.

   ```sh
   ❯ nargo test
   [hello] Running 1 test functions
   [hello] Testing test_main... ok
   [hello] All tests passed
   ```

9. Generate Solidity contract via `$ nargo codegen-verifier`:

   ```sh
   ❯ nargo codegen-verifier
   [hello] Contract successfully created and located at ~/hello/contract/hello/plonk_vk.sol
   ```

10. Get info:

```sh
❯ nargo info
+---------+------------------------+--------------+----------------------+
| Package | Language               | ACIR Opcodes | Backend Circuit Size |
+---------+------------------------+--------------+----------------------+
| hello   | PLONKCSat { width: 3 } | 5            | 5                    |
+---------+------------------------+--------------+----------------------+
```

## Resources

- [Noir W3C Verifiable Credentials](https://github.com/Noir-W3C-Verifiable-Credentials/protocol-core)
- [Awesome Noir](https://github.com/noir-lang/awesome-noir/tree/main#benchmarks)
- [ ] KZG polynomial commitment in Noir
- [ZKCamp's Aztec Noir Course](https://github.com/ZKCamp/aztec-noir-course) 🌟🌟🌟
  > Learn theory to practical coding.
