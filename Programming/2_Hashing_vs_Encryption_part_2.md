![learn cryptography.png](https://res.cloudinary.com/hpiynhbhq/image/upload/v1514338762/m5ifbwspszd0v2ggzvkg.png)

Hi Guys!!....

### This is the programming part of the previous tutorial ```"Hashing vs Encryption"``` in this **"Learn Cryptography"** series.

View Part 1 in [Steemit](https://steemit.com/utopian-io/@abhi3700/learn-cryptography-1-hashing-vs-encryption), [Github](https://github.com/abhi3700/My_Learning_Cryptography-Concepts/blob/master/Basics/1_Hashing_vs_Encryption.md)
In previous tutorial, some basics is covered regarding **Hashing** & **Encryption** and the main differences between them.

In this tutorial, we will be programming using **Python** for hashing any text with different hash algorithms.

## Tools Installation
We will be using [Anaconda](https://anaconda.org/) for accessing the features.
* **Create a separate environment**. It's not necessary but recommended so as to avoid conflicts in case of multiple type of projects running in a PC/ laptop.
```conda create --name learn-cryptography python=3```

* Now, when the notebook opens, **choose the correct shell** (where all tools are installed)
![3.png](https://res.cloudinary.com/hpiynhbhq/image/upload/v1514440688/cqqyfg8xnwpl6t6yppcx.png)

After choosing the "learn-cryptography" notebook it opens like this..
![8.png](https://res.cloudinary.com/hpiynhbhq/image/upload/v1514446363/sjbgykftpford828e6px.png)

* Now, **enter into the environment** so as to install the required libraries.
```activate learn-cryptography```
![1.png](https://res.cloudinary.com/hpiynhbhq/image/upload/v1514340029/pbehpltggcklkrwqw7kn.png)

* Install the nb_conda (specifically for an environment) using 
```conda install nb_conda```

* **Open the jupyter notebook** in your directory (where project files are to be saved)
Use ```jupyter notebook```
![2.png](https://res.cloudinary.com/hpiynhbhq/image/upload/v1514440641/dhavtbakldc99rosrfwf.png)


* Now, install the libraries for practising cryptography - 
1. *'cryptography'* - ```pip install cryptography```
![5.png](https://res.cloudinary.com/hpiynhbhq/image/upload/v1514445842/n5kupoexrgzl0b36ecq9.png)
![7.png](https://res.cloudinary.com/hpiynhbhq/image/upload/v1514446120/iytvf3f0bbzohtjhaswj.png)

2. **pycryptodome** - ```pip install pycryptodomex```. Just to avoid any conflict while *importing Crypto* library. Also , install this using ```pip install pycryptodome```.
![4.png](https://res.cloudinary.com/hpiynhbhq/image/upload/v1514445814/i0xn5r9umgbnsstgevsw.png)
![6.png](https://res.cloudinary.com/hpiynhbhq/image/upload/v1514445977/xxpntxjaqrfhxqrbf9nq.png)

Now, there is no conflict at all importing libraries. 

Let's start coding now....

## Coding
We are going to cover **Hashing** in this tutorial. And **Encryption-Decryption** shall be covered in the subsequent tutorial. In *encryption-decryption*, there are 2 main types - **symmetric** and **assymetric**. 

### Hashing
Let's start with hashing a string - "abc" using SHA256 method.
```python
from Crypto.Hash import SHA256
SHA256.new(b'abc').hexdigest()
```
![9.png](https://res.cloudinary.com/hpiynhbhq/image/upload/v1514449997/n3dacts9uqm4wieswsnu.png)
Here, string *'b'* in the input of the function **new()** represents byte string.

So, ```ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad``` is the hash for string ```abc``` using **SHA256** hash algorithm. And this is unique. Everytime this string will give the same output and no other input shall give this particular hash.

> In cryptography hash function,  any algorithm which fails the 2 properties (mentioned below) are not used anymore. And this happened with **MD5** algorithm-
> 1. **collision attacks** -  when 2 different inputs result in the same hash output. It was found in 2004 and 2008.
> 2. **preimage attacks** - when a hash 'h' is put into the **MD5** function and message m is obtained. where ```hash(m) = h``` that means the function is reversible which violates the property.

## Application
### Password management and Storage
Used for matching the **password input** with the hash of the **password stored in the database**.
Code snippet:
```python
from Crypto.Hash import SHA256

# a function defined for checking if correct password entered?
def check_password(input_password, actual_password):
    
    # here, SHA256.new() function input as bytes. So, used str.encode() function for 
    # encoding string into bytes.
    actual_password_hash = SHA256.new(str.encode(actual_password)).hexdigest() # hash of actual password
    input_password_hash = SHA256.new(str.encode(input_password)).hexdigest() # hash of input password
    
    return input_password_hash == actual_password_hash
```
Let's make a small application where one enters the password and its hash gets stored.
While entering the password next time, it will return TRUE or FALSE by inserting into
check_password() function.
```python
# save the actual password
a = input("Enter the actual password:") # runs 1 time.

# This loop ends only when the correct password is entered, otherwise it continues.
while(1):
    i = input("Enter your actual password again:")

    result = check_password(i, a)  # checks the password
    
    if result == True:
        print("Congrats!... Correct password.")
        break
    else:
        print("Sorry!... INCORRECT password\n\n")
    
# End of the code...
```
![10.png](https://res.cloudinary.com/hpiynhbhq/image/upload/v1514467424/febgnmjmokydf8v0onqr.png)

So, entering *wrong* returns FALSE i.e. 'Incorrect password'.

This is how the ```Login screen``` works in the back-end of the code.

That's all for now.

## References
* (Blog) Python and cryptography with pycrypto - https://www.laurentluce.com/posts/python-and-cryptography-with-pycrypto/
* (Blog) 
* (Tool) Python cryptography - [Github](https://github.com/pyca/cryptography) , [Website](https://cryptography.io/en/latest/)
* **Deprecated** (Tool) PyCrypto - [Github](https://github.com/dlitz/pycrypto), [Website](https://www.dlitz.net/software/pycrypto/) DON'T USE THIS...
* (Tool) PyCryptodome: A fork of PyCrypto (after version 2.6.1) - [Github](https://github.com/Legrandin/pycryptodome), [Website](https://pycryptodome.readthedocs.io/en/latest/) . Read the [blog](https://blog.sqreen.io/stop-using-pycrypto-use-pycryptodome/) which recommends using the new library as the old one is prone to hacking.
* Cryptographic Services with Python - https://docs.python.org/3/library/crypto.html
* Python 3: An Intro to Encryption - https://www.blog.pythonlibrary.org/2016/05/18/python-3-an-intro-to-encryption/
* 



