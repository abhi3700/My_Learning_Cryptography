![learn cryptography.png](https://res.cloudinary.com/hpiynhbhq/image/upload/v1514662777/gdtvefasejwtgqkvqeyu.png)

Hey guys!...

### This is the next tutorial on `"Hashing using Python"` where we will learn how to hash a file's content.

In the last tutorial, we learned about some basics on "Installation & how to hash a string". And its application on implementing login screen of any App.

In this session, it will covered on how to hash a text file, image, hybrid.

**Previous tutorials in **"Learn Cryptography"** series:**

1. [Learn Cryptography #1 - Hashing vs Encryption](https://steemit.com/utopian-io/@abhi3700/learn-cryptography-1-hashing-vs-encryption)
2. [Learn Cryptography #2 - Hashing using Python](https://utopian.io/utopian-io/@abhi3700/learn-cryptography-2-hashing-using-python)

## Introduction

In **cryptography**, the applications are not exterior but rather interior. By this I mean that the Apps contains cryptography fitted inside e.g.

- **Login screen using password (in string)** - Here, any password's hash is saved on to the cloud. (coding covered in [tutorial 2](https://utopian.io/utopian-io/@abhi3700/learn-cryptography-2-hashing-using-python)) .

- **File Integrity Checker** - checking the authenticity of the files (any - text, image, hybrid) to be downloaded by comparing the hashes of the original file & current file.
  As mentioned in [Wiki](https://en.wikipedia.org/wiki/File_integrity_monitoring):
  > File Integrity monitoring (FIM) is an internal control or process that performs the act of validating the integrity of operating system and application software files using a verification method between the current file state and a known, good baseline. This comparison method often involves calculating a known cryptographic checksum of the file's original baseline and comparing with the calculated checksum of the current state of the file. Other file attributes can also be used to monitor integrity.

For more details, read this paper - [An Introduction To File Integrity Checking
On Unix Systems](https://www.giac.org/paper/gcux/188/introduction-file-integrity-checking-unix-systems/104739)
And we are going to make one....

## Coding

- **Import the libraries** - importing the libraries for the syntax used below. 'os' is imported for file path. Rest are all hash algorithms. Any of them can be used for hashing pupose.

```python
import os

# Importing hash algorithms - MD5, SHA1, SHA224, SHA256, SHA384, SHA512
from Crypto.Hash import MD5
from Crypto.Hash import SHA1
from Crypto.Hash import SHA224
from Crypto.Hash import SHA256
from Crypto.Hash import SHA384
from Crypto.Hash import SHA512
```

![1.png](https://res.cloudinary.com/hpiynhbhq/image/upload/v1514662933/vqcavqdgc0hrt04auz1c.png)

- **Define methods to be used** - Here, 2 methods are defined. One calculates the file size and other is for getting the hash of file's content.
  _'rb'_ is for reading the bytes of the file.

File is read in bytes and then fed into the hash function (whichever - **MD5**, **SHA** family).

```python
# A function calculating the file size in bytes.
def get_file_size(fname):
    return os.path.getsize(fname)

# define a function that reads data from a file and hash it.
def get_file_hash(hash_algorithm):
    filename = input("Enter the filename with extension: ")
    h = hash_algorithm.new()
    chunk_size = get_file_size(filename) # size of file in bytes.

    # getting file's chunk in bytes. 'rb' indicates binary.
    with open(filename, 'rb') as fh:  # rb means file is read in binary  mode i.e. bytes as input for hashing
        while True:
            chunk = fh.read(chunk_size)
            if len(chunk) == 0:
                break
            h.update(chunk)
    return h.hexdigest()
```

![2.png](https://res.cloudinary.com/hpiynhbhq/image/upload/v1514663061/wz7kj7hsdsrs5bis0tdf.png)

- **Get the output** - converting a text, image, hybrid (both text, image) file into a hash.
  Code snippet is same for all the file formats.

```python
hash_file = get_file_hash(SHA256)
len_hash = len(hash_file)
print("Hash: " + hash_file + "\nand it's length is: " + str(len_hash) + " bytes" + " or " + str(len_hash*4) + " bit" )
```

**Case 1**: **Text file into hash** - a random text file chosen "new.txt" (contains few lines) is converted into a hash number.
![1.png](https://res.cloudinary.com/hpiynhbhq/image/upload/v1514664080/qjlrgcfbqrxjfppiraew.png)
![3.png](https://res.cloudinary.com/hpiynhbhq/image/upload/v1514664828/ddo21vf0am7nogw9bybk.png)

**Case 2**: **Image file into hash** - 'Bitcoin' icon (png format) is converted into hash.
![bitcoin.png](https://res.cloudinary.com/hpiynhbhq/image/upload/v1514664387/tjuffqqdtmbesvf7az33.png)
![4.png](https://res.cloudinary.com/hpiynhbhq/image/upload/v1514664796/wkglcluuwsmc84tawlmt.png)

**Case 3**: **Hybrid file into hash** - A pdf file chosen for converting into its hash number.
![2.png](https://res.cloudinary.com/hpiynhbhq/image/upload/v1514665003/b9f3mtwpir9ejlwhjc0c.png)
![5.png](https://res.cloudinary.com/hpiynhbhq/image/upload/v1514665017/jkd3cyhmllbnsbcosey0.png)

So, we have been able to convert any kind of files into its corresponding hash.

Whoa!!!.....we made a file integrity checker or monitoring .

That's all for now.

### Stay tuned for next tutorial.
