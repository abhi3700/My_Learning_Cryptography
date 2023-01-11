# Hashing vs Encryption

![learn cryptography.png](https://res.cloudinary.com/hpiynhbhq/image/upload/v1512301517/cl0gboe9cwzirgy7rjmy.png)

Hey Guys!!....

## In this tutorial series, we will be learning various **Cryptography** related concepts - both basic and core through examples.

The cryptography plays a major role in many day-to-day applications like Chat (Messaging) Apps, Password stored in database, Fetching the secure website i.e. **HTTPS**, etc. In many cases, we don't see the layer behind. It's actually cryptography used.

Now a days, we are seeing a lot of cryptocurrencies - Bitcoin, Ethereum, Litecoin, etc. In all these, the entire blockchain protocol (Mining algorithms) used is constructed using cryptography majorly. So, it's high time that we start learning about this subject.

## Hashing

### Concept

It is a technique of passing an input string (any size) into a **"Hash Function"** and generating a unique hash number (fixed length) out of it.

    ![](http://vignette1.wikia.nocookie.net/computersecuritypsh/images/5/5f/Hash_Function.png/revision/latest?cb=20110323192006)

E.g. of hash function - MD5, SHA-1, SHA-2 (mostly used), etc.

![](https://upload.wikimedia.org/wikipedia/commons/thumb/2/2b/Cryptographic_Hash_Function.svg/1200px-Cryptographic_Hash_Function.svg.png)

### Application

- While storing password in database - when user makes login attempt, the entered password is compared with the stored password (hash numbers compared in this case).
  ![cryptography_.png](https://res.cloudinary.com/hpiynhbhq/image/upload/v1512304732/ocbhj08odwz0h1bg6ouf.png)

- While entering the Credit card details, the last digits are compared with the stored credit card in the database.
  ![credit_card_back.gif](https://res.cloudinary.com/hpiynhbhq/image/upload/v1512305234/ffjaqcprr6o6mu8jzbah.gif)

> NOTE: Basically, whenever we want to save any plain text in hidden form, we can use **Hashing** for that purpose.

## Encryption

#### Concept

A technique where an input is converted into an unreadable output of any length.
Here, the input is retrievable through decryption using key.

**Types**- Symmetric & Assymetric.
**Symmetric** - The keys for encryption and decryption.
**Assymetric** - 2 different keys. **Public key** for encryption and **Private key** for decryption.

#### Application -

It is used where any information is supposed to be retrieved by the receiver.

- In Chat applications, the messages sent are to be read by the receiver. In this case, if **Hashing** used, it is of no use because the user won't be able to understand the text due to it's property of one-way (irreversible).
  In this, any message before sending is encrypted by sender using his **public-key** and the receiver retrieves the message and decrypts it using sender's **private-key**. Like this, the chat applications work in the behind.
  ![http_%2F%2Fmashable.com%2Fwp-content%2Fuploads%2F2014%2F11%2Fwhatsapp-message-kari.png](https://res.cloudinary.com/hpiynhbhq/image/upload/v1512306501/zydq664ppc1zoc4xtmlc.png)

## Difference

**Hashing** - non-retrievable input, used for storing and comparison.
**Encryption** - retrievable input, used where the input is to be read by the receiver e.g. Chat Apps.

That's all.

### Stay tuned for the next tutorial.

### A series on "Cryptography Programming with Python" is on the way.

Thanks for reading....

<br /><hr/><em>Posted on <a href="https://utopian.io/utopian-io/@abhi3700/learn-cryptography-1-hashing-vs-encryption">Utopian.io - Rewarding Open Source Contributors</a></em><hr/>

## References

- https://www.securityinnovationeurope.com/blog/page/whats-the-difference-between-hashing-and-encrypting
