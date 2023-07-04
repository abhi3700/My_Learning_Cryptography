# Encryption using Python

![](../img/learn_cryptography.png)

Hey Guys!!..

### In this tutorial, we will be learning **"Encryption using Python"**. Basically, encrypting any content using **Python** programming language.

We use encryption in many applications like - Chat apps, etc.

**Previous tutorials in "Learn Cryptography" series:**

1. [Learn Cryptography #1 - Hashing vs Encryption](https://steemit.com/utopian-io/@abhi3700/learn-cryptography-1-hashing-vs-encryption)
2. [Learn Cryptography #2 - Hashing using Python](https://utopian.io/utopian-io/@abhi3700/learn-cryptography-2-hashing-using-python)
3. [Learn Cryptography #3 - Hashing using Python - part 2 | "File Integrity Checker "](https://utopian.io/utopian-io/@abhi3700/learn-cryptography-3-file-integrity-checker-or-hashing-using-python-part-2)

Here, we will cover **Symmetric encryption**.

Follow the **installation** [here](https://utopian.io/utopian-io/@abhi3700/learn-cryptography-2-hashing-using-python)
There are 2 types of ciphers used - **Block** and **Stream**.

- Block - fixed size (8 or 16 bytes). see [Wiki](https://en.wikipedia.org/wiki/Block_cipher)
- Stream - byte-by-byte. see [Wiki](https://en.wikipedia.org/wiki/Stream_cipher)

### Block ciphers

Let's use the simplest algorithm first i.e. **DES**.

**Data Encryption Standard (DES)** is a symmetric encryption algorithm.
![des_structure.jpg](https://res.cloudinary.com/hpiynhbhq/image/upload/v1515421095/mtk0fblhpgsxfmhirmoy.jpg)

In the image above, it is described:

- uses 16 round [Fiestal](https://en.wikipedia.org/wiki/Feistel_cipher) structure (i.e. symmetric encryption)
- Plain-text --> 64-bit or 8-bytes (i.e. multiple of 8 in length, otherwise ERROR occurs).
- key --> 64-bit or 8-bytes (actually 56-bit, rest 8 bits are not used).
- Cipher-text --> 64-bit or 8-bytes.

![](https://i.imgsafe.org/31/3132b80dff.png)

#### Installation

Install an additional library for DES algorithm using pip in cmd as follows:
`pip install pydes`
For more, refer [here](https://github.com/toddw-as/pyDes)

> **Performance**:The code (of this package) is not written for speed or performance, so not for those needing a fast DES implementation, but rather a handy portable solution ideal for small usages. The speed at which pyDes encrypts/decrypts is around 10Kb/s (using the DES method) - that's very SLOW!!

#### Coding

```python
# import the DES library using a customized package
import pyDes
```

```python
# input
i = input("Enter any string: ")

# Padding function: add ' ' until the string length is multiples of 8
def padded_text(s):
    while len(s)%8 !=0 :
        s += ' '
    return s

p = padded_text(i)
```

![1.png](https://res.cloudinary.com/hpiynhbhq/image/upload/v1515421314/suthu3inlkov9rvpyeic.png)

There are different modes of generating keys -

##### [ECB](<https://en.wikipedia.org/wiki/Block_cipher_mode_of_operation#Electronic_Codebook_(ECB)>)

> The message is divided into blocks, and each block is encrypted separately.

![](https://i.imgsafe.org/36/36df8956d5.png)
![](https://i.imgsafe.org/36/36e7017b5b.png)

```python
# key should be 8 bytes long.
k_ecb = pyDes.des("DESCRYPT", pyDes.ECB, "\0\0\0\0\0\0\0\0", pad=None, padmode=None)

# encrypted data i.e. in bytes
e_ecb = k_ecb.encrypt(str.encode(p))
print("\n The encrypted string(in bytes) - \n", e_ecb)

# extract the input text from the encrypted input using decryption
d_ecb = k_ecb.decrypt(e_ecb)
print("\n The actual input(in bytes) -  \n", d_ecb)
```

![2.png](https://res.cloudinary.com/hpiynhbhq/image/upload/v1515421473/ikhafafr4piy65chdutd.png)

##### [CBC](<https://en.wikipedia.org/wiki/Block_cipher_mode_of_operation#Cipher_Block_Chaining_(CBC)>)

> each block of plaintext is XORed with the previous ciphertext block before being encrypted.

![](https://i.imgsafe.org/37/3710b5f07b.png)
![](https://i.imgsafe.org/37/371537706a.png)

```python
# key should be 8 bytes long. IV vector given some value.
k_cbc = pyDes.des("DESCRYPT", pyDes.CBC,"\0\0\0\0\0\1\0\0" , pad=None, padmode=None)

# encrypted data i.e. in bytes
e_cbc = k_cbc.encrypt(str.encode(p))
print("\n The encrypted string(in bytes) - \n", e_cbc)

# extract the input text from the encrypted input using decryption
d_cbc = k_cbc.decrypt(e_cbc)
print("\n The actual input(in bytes) -  \n", d_cbc)
```

![3.png](https://res.cloudinary.com/hpiynhbhq/image/upload/v1515421557/q0lbicd4bgzb5rhm6dms.png)

#### NOTE: Now a days, its key length is too short. And also not secure as it can be brute-forced with some effort.

For more, read [here](http://www.freeswan.org/freeswan_trees/freeswan-1.5/doc/DES.html).

**Triple DES (3DES)** is an improved version of **DES**.
It is secure. key-length is long in this case.
See [Wiki](https://en.wikipedia.org/wiki/Triple_DES)

> applies the DES cipher algorithm three times to each data block.

That's it for now.

### Stay tuned for more such content...
