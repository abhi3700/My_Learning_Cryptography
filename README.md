# My_Learning_Cryptography-Concepts

Learn about cryptography concepts - Both basics and core covered here.

![cryptography_.png](./res/learn%20cryptography.png)

## Concepts

- **RSA vs ECC**
  - RSA is based on integer factorization problem, while ECC is based on discrete logarithm problem.
  - RSA has lower key size than ECC.
  - RSA key generation is slower than ECC, but verification of the former is faster than the latter => 20000 RSA vs 8000 ECC key verification.
  - RSA is simpler to implement than ECC, also the former is less expensive to implement.
    > Now, that quantum computer research has advanced, ECC is more secure than RSA, but it still can be broken by quantum computers. So, the researchers are working on quantum resistant cryptography.
- Generating a **private key from a mnemonic passphrase** is a two-step process:

  1. **Seed generation**: There is a <kbd>mnemonic passphrase (12 to 24 words)</kbd> -> <kbd>512-bit seed</kbd> using the BIP-39 algorithm.
  2. **Private key generation**: <kbd>512-bit seed</kbd> seeded with <kbd>PRNG</kbd> -> <kbd>256-bit private key</kbd>

  > This private key can then be used for a wide variety of purposes such as creating digital signatures, generating public/private key pairs and deriving key pairs for encryption.

## Books

- Hacking using python - https://inventwithpython.com/hacking/
- Handbook of Applied Cryptography - http://cacr.uwaterloo.ca/hac/

## Software tools

- Python Crypto Cheatsheets - https://www.pythonsheets.com/notes/python-crypto.html
- Python cryptography - [Github](https://github.com/pyca/cryptography) , [Website](https://cryptography.io/en/latest/)
- **Deprecated** PyCrypto - [Github](https://github.com/dlitz/pycrypto), [Website](https://www.dlitz.net/software/pycrypto/) DON'T USE THIS...
- PyCryptodome: A fork of PyCrypto (after version 2.6.1) - [Github](https://github.com/Legrandin/pycryptodome), [Website](https://pycryptodome.readthedocs.io/en/latest/) . Read the [blog](https://blog.sqreen.io/stop-using-pycrypto-use-pycryptodome/) which recommends using the new library as the old one is prone to hacking.
- pyDes: a library for DES symmetric encryption algorithm - [Github1](https://github.com/toddw-as/pyDes), [Github2](https://github.com/RobinDavid/pydes), [pip package](https://pypi.python.org/pypi/pyDes/)
- pyaes: a library for AES symmetric encryption algorithm - [Github](https://github.com/ricmoo/pyaes).
- Python-RSA: a library for RSA i.e. assymetric algorithm - [Github](https://github.com/sybrenstuvel/python-rsa)

## Resources

- (Blog) Useful Cryptography Resources - https://blog.cryptographyengineering.com/useful-cryptography-resources/
- (Blog) RSA Algorithm in Cryptography - https://www.geeksforgeeks.org/rsa-algorithm-cryptography/
- (Blog) Adding salt to Hashing - https://auth0.com/blog/adding-salt-to-hashing-a-better-way-to-store-passwords/
- (Blog) Elliptic Curve Cryptography - https://medium.com/coinmonks/elliptic-curve-cryptography-6de8fc748b8b
