# Cairo

## About

- A language for writing zk-STARKs maths circuits.
- Analogous to Circom language (for zkSNARK).
- Previously, Cairo was built on top of Python
- Now, Cairo is very bullish with rust variant like Circom (Rust).

## Installation

> For macOS M1.

### Cairo [Rust]

#### CLI

##### Package manager

[SOURCE](https://docs.swmansion.com/scarb/docs/install#quick-installation)

```console
❯ curl --proto '=https' --tlsv1.2 -sSf https://docs.swmansion.com/scarb/install.sh | sh                                                                                            ⏎
scarb-install: retrieving latest version from https://github.com/software-mansion/scarb...
scarb-install: downloading scarb-v0.5.0-aarch64-apple-darwin.tar.gz...
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
100 19.9M  100 19.9M    0     0  6316k      0  0:00:03  0:00:03 --:--:--  9.7M
scarb-install: installed scarb to /Users/abhi3700/.local/share/scarb-install/latest
scarb-install: created symlink /Users/abhi3700/.local/bin/scarb -> /Users/abhi3700/.local/share/scarb-install/latest/bin/scarb

Detected your preferred shell is zsh and added '$HOME/.local/bin' to PATH. Run 'source /Users/abhi3700/.zshrc' or start a new terminal session to use Scarb.
Then, run 'scarb --version' to verify your installation. Happy coding!
```

```console
source /Users/abhi3700/.zshrc
```

Verify the installation:

```console
$ scarb --version
```

##### Language server

Using `scarb` package manager.

[Source](https://github.com/starkware-libs/cairo/blob/main/vscode-cairo/README.md)

#### Editor

VSCode Extensions:

- [vscode-cairo](https://github.com/starkware-libs/cairo/tree/d485f5ffd0c444d900cdcac57b9e745dcc280fba/vscode-cairo)

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
