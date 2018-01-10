![learn cryptography.png](https://res.cloudinary.com/hpiynhbhq/image/upload/v1515498394/qkmdtmtmcizr38aw3ffx.png)

Hey Guys!!..
### In this tutorial, we will be learning further about "Encryption using Python". We will continue from where we ended in the last tutorial.

**Previous tutorials in this "Learn Cryptography" series:**
1. [Learn Cryptography #1 - Hashing vs Encryption](https://steemit.com/utopian-io/@abhi3700/learn-cryptography-1-hashing-vs-encryption)
2. [Learn Cryptography #2 - Hashing using Python](https://utopian.io/utopian-io/@abhi3700/learn-cryptography-2-hashing-using-python)
3. [Learn Cryptography #3 - Hashing using Python - part 2 | "File Integrity Checker "](https://utopian.io/utopian-io/@abhi3700/learn-cryptography-3-file-integrity-checker-or-hashing-using-python-part-2)
4. [Learn Cryptography #4 - Encryption using Python | "Symmetric" | "Block Cipher"](https://utopian.io/utopian-io/@abhi3700/learn-cryptography-4-encryption-using-python-or-symmetric-or-block-cipher)

Here, we will cover the **Stream cipher** which is another type of  **"Symmetric Encryption"**.

## Introduction
[Source](http://searchsecurity.techtarget.com/definition/stream-cipher)
> A stream cipher is a method of encrypting text (to produce ciphertext) in which a cryptographic key and algorithm are applied to **each binary digit in a data stream, one bit at a time**. This method is not much used in modern cryptography. The main alternative method is the block cipher in which a key and algorithm are applied to blocks of data rather than individual bits in a stream.

For more, read [Wiki](https://en.wikipedia.org/wiki/Stream_cipher)

**Applications: ** 
Read this [PAPER](https://pdfs.semanticscholar.org/83bf/cf8cf702a1ae73e470d9370de6d1c33d6cdf.pdf)
>  Stream cipher is usually used in limited resource
environment, such as cell phones, network stream media,
wireless network and mobile devices and so on.

**Advanced Encryption Standard (AES)** is an algorithm, successor of **DES** (discussed in the previous [tutorial](https://utopian.io/utopian-io/@abhi3700/learn-cryptography-4-encryption-using-python-or-symmetric-or-block-cipher)). It was invented due to the following 2 reasons:
* there was some security problem in **DES** i.e. attack through brute-force.
* AES is based on a design principle known as a substitution-permutation network, a combination of both substitution and permutation, and is fast in both software and hardware. For more, read [this](https://www.schneier.com/academic/paperfiles/paper-twofish-final.pdf)

Features:
* **key size** - 128-bits (16-bytes), 192-bits (24-bytes) or 256 bits (32-bytes).
* **block size** - 128 bits
* **rounds** - 10, 12 or 14 (depending on key size) respectively.

> **AES** has been adopted by the U.S. government and is now used worldwide. It supersedes the **Data Encryption Standard (DES)**, which was published in 1977. The algorithm described by AES is a symmetric-key algorithm, meaning the same key is used for both encrypting and decrypting the data. [Source](https://en.wikipedia.org/wiki/Advanced_Encryption_Standard)

Here, we will use the **AES** algorithm (CTR mode) for stream cipher. For more, read [AES as A Stream Cipher](https://pdfs.semanticscholar.org/83bf/cf8cf702a1ae73e470d9370de6d1c33d6cdf.pdf)
>  As a well-known alternative, by feeding back its
key stream, block cipher could be adopted as a stream cipher.
So in this paper, we use Counter Mode (CTR) AES to make it
as a stream cipher. 

## Coding
```python
# import the AES library
import pyaes
```

```python
# Enter plain text of any byte (stream)
i = input("Enter the plain text: ")  

# A 256 bit (32 byte) key chosen
key = "abhijit#4387926131r513f124597851"

# CTR mode chosen for stream cipher
aes = pyaes.AESModeOfOperationCTR(str.encode(key))

# cipher text creation
e = aes.encrypt(i)
# or, use this directly
# e = pyaes.AESModeOfOperationCTR(str.encode(key)).encrypt(i)

print("\n The Encrypted text (in bytes): \n", e)


# decrypting cipher text
# The counter mode of operation maintains state, so decryption requires a new instance be created
aes = pyaes.AESModeOfOperationCTR(str.encode(key))
d = aes.decrypt(e)
# or, use this directly
#d = pyaes.AESModeOfOperationCTR(str.encode(key)).decrypt(e)

print("\n The Decrypted text (in bytes): \n", d)
```
![3.png](https://res.cloudinary.com/hpiynhbhq/image/upload/v1515600171/ynhhb8okt4ebibycy504.png)

Here, in the example above we give the plaintext **"abhijit"** & encrypt it and then decrypt. The plaintext can be of any bytes  as we are using the **AES as a stream cipher** in CTR mode.

##### NOTE: The counter mode ```aes = pyaes.AESModeOfOperationCTR(str.encode(key))``` has to be initialised during encryption & decryption as well.

Yet, we have not used the initial counter value in the CTR mode while creating the **AES** key for encryption.
We will do that in the following code.

[Source](https://www.cryptopp.com/wiki/CTR_Mode)
> CTR is counter mode. CTR mode was standardized in 2001 by NIST in SP 800-38A. CTR mode uses a counter rather than a traditional IV. The counter has additional properties, including a nonce and initial counter block. The mode does not require padding the plain text to the block size of the cipher.

**Code snippet**
```python
# Enter plain text of any byte (stream)
i = input("Enter the plain text: ")  

# A 256 bit (32 byte) key chosen
key = "abhijit#4387926131r513f124597851"

# To use a custom initial value
counter = pyaes.Counter(initial_value = 100)

# CTR mode chosen for stream cipher
aes = pyaes.AESModeOfOperationCTR(str.encode(key), counter= counter)

# cipher text creation
e = aes.encrypt(i)
# or, use this directly
# e = pyaes.AESModeOfOperationCTR(str.encode(key)).encrypt(i)

print("\n The Encrypted text (in bytes): \n", e)


# decrypting cipher text
# The counter mode of operation maintains state, so decryption requires a new instance be created
counter = pyaes.Counter(initial_value = 100)
aes = pyaes.AESModeOfOperationCTR(str.encode(key), counter= counter)
d = aes.decrypt(e)
# or, use this directly
#d = pyaes.AESModeOfOperationCTR(str.encode(key), counter= counter).decrypt(e)

print("\n The Decrypted text (in bytes): \n", d)
```
![4.png](https://res.cloudinary.com/hpiynhbhq/image/upload/v1515600451/tqn6bqby7p75wpv0extv.png)

Here, we did the same thing i.e. feeding plaintext and decrypting the encrypted text / ciphertext.

##### NOTE: The counter value has to be initialised during encryption & decryption as well. i.e.
```python
counter = pyaes.Counter(initial_value = 100)
aes = pyaes.AESModeOfOperationCTR(str.encode(key), counter= counter)
```

## Conclusion
We saw that **AES** can be used as a stream cipher which is the main purpose of this tutorial.
Following points to be pondered upon:
* **AES in CTR mode** can be used for stream cipher
* Like **IV** initialisation vector in block cipher here, we have used the counter value e.g. 100

That's all for now.

### Stay tuned for more such tutorials...
