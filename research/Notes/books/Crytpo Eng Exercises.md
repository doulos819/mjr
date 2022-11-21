# Crytpo Eng Exercises

> Ex for [[Cryptography Engineering - 2010]]

### Chapter 1
1. Create an attack tree for stealing a car.
2. Create attack tree for getting into gym without paying.
3. Create attack tree for getting food from restaurant without paying.
4. Create attack tree for learning someone's online banking account name and password.
5. Create attack tree for reading someone else's e-mail.
6. Create attack tree for preventing someone from reading their own e-mail.
7. Create attack tree for sending e-mail as someone else.
8. Find a new product or system that was announced or released recently. Construct a security review as described in Sec 1.12. Pick one asset, and construct attack tree for compromising that asset.
9. Provide concrete example in which attackers compromised a system by exploiting something other than the weakest link. Describe the system, what was the weakest link and why, how was the system compromised.
10. Describe a concrete example where improving the security of a system against one type of attack can increase the likelihood of other attacks
	- diffusion topic define for uncloak: https://uncloak.org/hash+functions/Term-Digest


### Chapter 2
1. Consider [Kerckhoffs' principle](https://en.wikipedia.org/wiki/Kerckhoffs%27s_principle). Provide at least two arguments in favor and two against. Then state/argue your view of the validity of the principle. 
2. Suppose Alice and Bob are sending e-mails to each other over the Internet from laptops connected to free wifi.
	1. List all the parties who might be able to attack this system and what they might be able to accomplish.
	2. Describe how Alice and Bob might be able to defend against each.
3. Consider a group of 30 people who wish to establish pair-wise secure communications using symmetric-key cryptography. How many keys need to be exchanged in total. 
	- A - 435 ($N(N-1)/2$)) 
4. Suppose Bob receives a messages signed using a digital signature scheme with Alice's secret signing key. Does it prove that Alice saw the message and chose to sign.
	- no, $K_a$ is long and difficult to remember so computer does the signing. Malicious party could have taken over Alice's PC and signed. 
5. Suppose that PKIs do not exist. Suppose Alice obtains a pubic key $P$ that purportedly belongs to Bob. How can Alice develop confidence that $P$ really belongs to Bob? Consider the following:
	- Alice can talk with Bob over the phone.
	- Alice can talk with someone else she trusts over the phone (Charlie), and they have already verifies that $P$ belongs to Bob.
	- Alice can communicate with Charlie via e-mail, and Charlie has already verified that $P$ belongs to bob.
> explicitly state any addition assumptions
6. Suppose a chosen-ciphertext attacker cannot recover the secret decryption key for an encryption scheme. Does this mean the encryption scheme is secure?
	- no, the attacker may be able to gain other info about the message (first bit) which could still help the attacker gain important info.
7. Consider a symmetric-key cryptosystem in which cryptographic keys are randomly selected from the set of all n-bit strings. Approximately what should n be in order to provide 128 bits of security against a birthday attack. 
	- $2^{64}$ 

### Chapter 3
1. How much space would be required to store a table for an entire idealized block cipher that operates on 64-bit blocks and has 80-bit keys?
2. How many rounds are in $DES$? How many bits are in a $DES$ key? What is the $DES$ block size? How does $3DES$ work as a function of $DES$?
3. What are the possible lengths for $AES$ keys? What is the $AES$ block size?
4. Under what situations might you choose $3DES$ over $AES$?
5. Suppose you have a processor that can perform a single $DES$ encryption or decryption operation in $2^{-26}$  seconds. Suppose you also have a large number of plaintext-ciphertext pairs for $DES$ under a single unknown key. How many hours would it take, on average, to find that $DES$ key, using an exhaustive search approach and a single processor? How many hours would it take, with a collaction of $2^{14}$ processors?
6. Consider a new block cipher, *DES2*, that consists only of two rounds of the *DES* block cipher. *DES2* has the same block and key size as *DES*. For this question you should consider the *DES* $F$ function as a black box that takes two inputs, a 32-bit data segment and a 48-bit round key, and that produces a 32-bit output. Suppose you have a large number of plaintext-ciphertext pairs for *DES2* under a single, unknown key. Given an algorithm for recovering the 48-bit round key for round 1 and the 48-bit round key for round 2. Your algorithm should require fewer operations than an exhaustive search for an entire 56-bit *DES* key. Can your algorithm be converted into a distinguishable attack against *DES2*?
7. Describe an example system that uses *DES* but is insecure because of the *DES* complementation property. Specifically, describe the system, and then present an attack against the system; the attack should utilize the *DES* complementation property. 
8. Familiarize yourself with a cryptographic software development package for your computer. A popular open source package is [*OpenSSL*](https://docs.rs/openssl/latest/openssl/aes/index.html).
	- Using an existing cryptographic library, decrypt the following ciphertext (in hex)
	```rust
	53 9B 33 3B 39 70 6D 14 90 28 CF E1 D9 D4 A4 07
```
with the following 256-bit key (also in hex)
```rust
	80 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 
	00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 01
	```
using *AES*.

9. Using an existing cryptography library, encrypt the following plaintext (in hex)

```rust
	29 6C 93 FD F4 99 AA EB 41 94 BA BC 2E 63 56 1D
```
with the following 256-bit key (also in hex)
```rust
	80 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 
	00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 01
```
using *AES*.

10. Write a program that experimentally demonstrates the complementation property for *DES*. This program should take as input a key $K$ and a plaintext $P$ and demonstrate that the $DES$ complementation property holds for this key and plaintext. You may use an existing cryptography library for this exercise.  