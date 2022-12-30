# Notes on the Text

###### tags: `cryptography` `uncloak`
Author(s): Niels Ferguson, Bruce Schneier, and Tadayoshi Kohono

Paper(s): 
- [Link to Text](https://drive.google.com/drive/folders/1506sz7G5o6ATeGObP1AEwMV4msaLK3HD?usp=sharing)

Link to exercises: https://github.com/doulos819/mjr/blob/main/research/Notes/books/Crytpo%20Eng%20Exercises.md

### Table of Contents
[[Object-Cryptograpgy Engineering#Part 1: Introduction |Part 1: Intro]]
- [[Object-Cryptograpgy Engineering#Ch 1. The Context of Cryptography |Ch 1 - Context of Crypto]]
- [[Object-Cryptograpgy Engineering#Ch 2. Introduction to Cryptography |Ch 2 - intro to Crypto]]
[[Object-Cryptograpgy Engineering#Part 2: Message Security |Part 2: Message Security]]
- [Ch 3. Block Ciphers](Cryptography%20Engineering%20-%202010#Ch%203.%20Block%20Ciphers)
- [Ch 4. Block Cipher Modes](Cryptography%20Engineering%20-%202010.md#Ch%204.%20Block%20Cipher%20Modes)
- [Ch 5. Hash Functions](#Ch%205.%20Hash%20Functions)
- [Ch. 6 Message Authentication Codes](#Ch.%206%20Message%20Authentication%20Codes)
- [Ch. 7 TLS/SSL - Secure Channels](week-4.md)
- [Ch. 8 Implementation Issues](#Ch.%208%20Implementation%20Issues)
[Part 3: Key Negotiation](#Part%203:%20Key%20Negotiation)
- [Ch 9. Generating Randomness](#Ch%209.%20Generating%20Randomness)

>[!Abstract]
>Most books cover what cryptography is, but this book provides invaluable resources for anyone working with cryptography. Cryptography and security engineers need to know more than just how protocols work; they need to know how to USE CRYPTOGRAPHY! 
>
>To know how to use cryptography, one must learn how to think like a cryptographer. 
>
>Both computer security and cryptography are about designing and evaluating objects (systems or algorithms) intended to behave in certain ways even in the presence of an adversary. 
>
>Cryptography on world stage (1990s) to secure internet; still no effective way for individuals to leverage. 


## Part 1: Introduction
In this Part:
- Chapter 1: The Context of Cryptography
- Chapter 2: Introduction to Cryptography

## Ch 1. The Context of Cryptography 
> Cryptography is the art and science of encryption, although also covers much broader set of elementary security functions. This book is designed to help cultivate mentality for thinking about security problems & a scientific background. 

First Lesson: Keep a critical mind; don't blindly trust anything.

### 1.1 The Role of Cryptography
> Cryptography by itself is fairly useless 

- Needs to be part of a larger system
- Similar to lock in real world; if rest of system is insecure, rest of system will get attacked before lock
- If attack on system, probably noticeable; attack on cryptography, probably not noticeable (re-entry possible/likely)

### 1.2 The Weakest Link Property
> A security system is only as strong as its weakest link.

- To improve the security of a system, we must improve the weakest link.
    - Strictly speaking, strengthening anything but the weakest link is useless.
    - In practice attacker may not know weakest link and attack stronger one.
- Attack tree can be used to ogranize links and better understand vulnerabilities in a system.
    - ![](https://i.imgur.com/y8Ro3Wv.png)

- Adding security to the wrong part of a system can lessen the overall efficiency/security of a system.
- Weakest link depends on situation; attacker has their own goals (which can change).
- *defence in depth*: property where strengething multiple links so remaining links can still provide security even if one link fails.

### 1.3 The Adversarial Setting
> Out opponents are intelligent, clever, malicious, and deviois; they'll do things nobody has every thought of before.

- Very harsh environment to work in
- No rules and deck stacked against us
- we don't know who she ("attacker") is, what she knows, what he goal is, when she will attack, or what her resources are. 
- Even with all time and resources available, she only needs to find one weak spot in system; we have to protect all areas.

### 1.4 Professional Paranoia
> To work in this field, you have to become devious yourself; think like a malicious attacker to find weaknesses in your own work. 

- Cryptographers are professional paranoids
    - Important to seperate professional / real-world paranoia to not go crazy
    - maybe this is possible..
- professional paranoia / paranois model -> security mindset 

#### 1.4.1 Broader Benefits 
> Once you develop a sense of professional paranoia, you will never look at systems the same way.

- This will benefit you throughout your career (cryptographer or not)
- Ability to identify secirity issues is important; even if issues cant be fixed by yourself, can get help.

#### 1.4.2 Discussing Attacks
> Very strict destinction between attacking someones work and attacking them personally.

- If someone proposes something; automatic invitation to attack.
- Break one of our systems, we will appload and tell everyone.
- This is the only way to learn how to make more secure systems.
- An attack on your work is not an attack on you.
- example "attack":"Have you thought about what might happen if you did this?" (then ->?)
    - Ease into discussion about attack.
    - Compliment on other security strengths.
    - Observer challenges to building secure systems.
    - Offer to help fix security problems if possible. 

### 1.5 Threat Model
> Every system can be attacked. There is no such thing as perfect security. 

- The point of a security system is to provide access to some people and not to others.
- In the end, you will always have to trust some people in some way, and these people may still be able to attack your system.
- What are you trying to protect, and against whom?
- A system is *"secure"*: It provides a sufficient level of security for our assets of interest against certain classes of threats.
- For any real system you should never forget the threat analysis for each of the participants. 

### 1.6 Cryptography Is Not The Solution
> It might be part of the solution, or it might be part of the problem. 

Cryptography can also make systems weaker when use din inappropriate ways. In many situations, it provides only a feeling of security, but no actual security.

### 1.7 Cryptography Is Very Difficult
> The weakest-link property and the adversarial setting conspire to make life for a cryptographer-or any security engineer-very hard.

There are some small areas of cryptography that are generally well understood, but that doesn't mean they are simple. These areas have just been worked on for a few decades and general consensus is that critical issues are known.

### 1.8 Cryptography Is The Easy Part
> Like a lock, a cryptographic component has fairly well-defined boundaries and requirements. 

Cryptography is the easy part, because there are people who know how to do a reasonably good job. 
- key management and key storage is crucial to any cryptographic system yet most computers have no secure place to store a key
- mobile devices...

### 1.9 Generic Attacks
> Some security problems can't be solved. 

- It is important to identify what the generic attacks against a system are
- generic attack from outside the system cannot be protected against (by any technology).

### 1.10 Security And Other Design Criteria
> Security is never the only design criterion for a system. Instead security is but one of many criteria.

#### 1.10.1 Security Versus 
> Good engineering lasts the tests of time.

- The priorities of computer industry largely value speed > security
- The result is generally a system that is somewhat efficient, yet is not sufficiently secure.
- "If it fails, fall back off and become more conservative"
- Lack of computer security is a hindrance to most users.
- Digital crooks of the future will have more resources and the problem will be larger
- "Good security is always a mixture of prevention, detection, and response"[114] 
- We are willing to spend up to 90% of CPU on security, but costs of security can many times be hidden from user.
- Most of the time. the overhead increase from an efficient security system shouldn't be noticeable on a human time scale; even when it's significant, that's just the cost of doing business.
- Security 1st, 2nd, 3rd, performance somewhere down the list. 
- Often realities of market > need for security. 

#### 1.10.2 Security Versus Features
> Complexity is the worst enemy of security.

- A computer program with 20 different binary options (on/off) has more than a million configurations.
    - To get the program to work: test most common configurations.
    - To make program secure: must evaluate each of the million possible configurations.
- To make a large, simple system you have to create a clear and simple interface between different parts of the system.
    - Complexity is a measure of how many things interact at any one point.

#### 1.10.3 Security Versus Evolving Systems
> One of the largest issues for security is that the system continues to evolve after the security mechanism is put in place. 

Designers need to consider a wider ranger of attackers and attack goals, while also anticipating and preparing for future uses of the system.

### 1.13 General Exercises
- [Crytpo Eng Exercises](https://github.com/doulos819/mjr/blob/main/research/Notes/books/Crytpo%20Eng%20Exercises.md)

## Ch 2. Introduction to Cryptography
> This chapter introduces basic cryptography concepts and provides background information for the book.

### 2.1 Encryption
- Encryption is the original goal of cryptography.
- A and B want to communicate, but E is eavesdropping. 
- Q: How can A & B communicate securely? 
- A & B agree on secret key $K_e$ 
- ![[Pasted image 20221024115132.png]]
- A encrypts $m$ before sending using and encryption function. 
	- $E(K_e,m)$ = c
	- c - *ciphertext* | m - *message* 

#### 2.1.1 Kerckhoff's Principle
> the security of the encryption scheme must depend only on the security of the key $K_e$, and not the security of the algorithm.    

- Algos are difficult to change; built into software/hardware, which can be difficult to update. 
- hard enough to keep a key secret, more difficult & more expensive to keep algo secret. 
- The authors of this book analysed "quite a number" of secret encryption algos, and *all* of them had weaknesses. 
- Don't trust security by obscurity

### 2.2 Authentication
> Eve (malicious) can do more than just listen, they can also change the message in some way $m$ -> $m'$ 

- Fig 2.3 How does Bob know who sent the message![[Pasted image 20221026104853.png]]
- Like encryption, authentication uses a secret key that Alice and Bob both know - $K_a$ 
- Fig 2.4 Generic setting for auth![[Pasted image 20221026105945.png]]
- Alice computes the MAC $a$ as $a:=h(K_a,m)$, where $h$ is the MAC function and $K_a$ is the authentication key.
	- Alice -> $m,a$ Bob
	- Bob recomputes $a$ using $K_a$ and checks to see if received $a$ is correct.
- A good MAC function will not give the same result for $h(K_a,m)$ and $h(K_a,m')$
- Eve can listen to Alice as she sends messages to record MACs (then Eve has $a$)
- Authentication is only part of the solution, Eve can still:
	- Delete messages from Alice -> Bob
	- Repeat old messages or change message order
	- Auth usually combined with numbering/nonce
- $f(K_e,m)$ doesn't stop manipulation of m contents.
- $h(K_a,m)$ doesn't keep a message secret. 

### 2.3 Public-Key Encryption
> How do Alice and Bob share $K_e$ (keys)?

- The problem of distributing and managing keys is one of the really difficult parts of cryptography, for which there are only partial solutions.
- If A and B are in a group of 20 that want to communicate, each member must exchange 19 keys; a total of 190 keys.
- Always assume that all communications (*$m$) are accessible to an enemy like Eve. 
- Generic setting for public-key encryption![[Pasted image 20221026134641.png]]
- To set things up Bob generates a pair of keys $(S_{Bob},P_{Bob})$ using a special algorithm.
- $D(S_{Bob},E(P_{Bob},m)) = m$ must hold for all possible messages $m$. 
- Now Bob only has to distribute a single public key that everybody can use. (Alice does the same)
- In practical systems that use public-key cryptography, one almost always sees a mixture of pub-key & secret-key algorithms.
- The pub-key algos are used to establish a secret key, which in turn is used to encrypt the actual data; combines flexibility pub-key crypto with the efficiency of symmetric-key crypto. 

### 2.4 Digital Signatures
> Digital signatures are the public-key equivalent of message authentication codes (MAC for symmetric key)

- Generic setting for digital signature![[Pasted image 20221026141356.png]]
- This time Alice used a key gen algo to gen a pair $(S_{Alice}, P_{Alice}))$ 
- When she wants to send $m$ to Bob, she computes a signature $s := \sigma (S_{Alice}, m)$ and sends $m, s$ to Bob
- Bob uses $v(P_{Alice}, m, s)$ that uses Alice's pub=key to verify the signature. 
- Anyone can use $P_{Alice}$ to verify that the message came from Alice. 

### 2.5 PKI
> Public Key Infrastructure: Central Authority (Certificate Authority/CA)

- Each user takes their pub-key to CA to self identify.
- CA then signs users pub-key using a digital signature; $P_{Bob}$ belongs to Bob.
- User must know CA pub-key + have the CA verify their pub-key; register once, use everywhere. 
- VeriSign is probably the best-known CA*(2010)
	- limited liability $100

### 2.6 Attacks
> Attacks on encryption schemes; there are many types, each with its own severity.


#### 2.6.1 The Ciphertext-Only Model
> A *ciphertext-only attack* is what most people mean when talking about breaking an encryption system.

- Trying to decrypt $m$ if you only know $c$ is called ciphertext-only attack.
- Most difficult type of attack because least amount of information. 

#### 2.6.2 The known-Plaintext Model
> A *known-plaintext attack* is one in which you know both the plaintext and the ciphertext 

- The most obvious goal is to find the decryption key; this will allow attacker to decrypt all other messages between Alive and Bob.
- More powerful than ciphertext-only attack; attacker gets more information.

#### 2.6.3 The Chosen-Plaintext Model
> Next level of control is for attacker to choose plaintext.

- Attacker gets to select specially prepared plaintexts; any number of them.
- There are two variations to this attack:
	- Offline attack: attacker prepares a list of all plaintexts they want to have encrypted before they get the ciphertexts.
	- Online attack: attacker can choose new plaintexts based on the ciphertexts they have already received.  
	- Online version is more powerful of the two.

#### 2.6.4 The Chosen-Ciphertext Model
> Should really be called: "Chosen ciphertext and plaintext attack"

- Attacker gets to choose both plaintext and ciphertext values. 
- More powerful because attacker has more freedom.
- Any reasonable encryption scheme has no trouble surviving a chosen ciphertext attack. 

#### 2.6.5 The Distinguishing Attack Goal
> There are too many attacks to list, new attacks thought up all the time. We wish to defend against a *distinguishing attack*

- A *distinguishing attack*: any nontrivial method that detects a difference between the ideal encryption scheme and the actual one.*
- This covers all the areas of attack discussed above, as well as yet-to-be discovered attacks.

#### 2.6.6 Other Types of Attack
> Attackers can attack other cryptographic functions such as authentication, digital signatures, ect.

- *information leakage* or *side-channel* attacks: Attacks that make used of additional information revealed from when ciphertexts were generated or 
 how fact encryption/decryption operations were can reveal private information about encrypted messages. 

### 2.7 Under the Hood
> Looking at two generic attacks

#### 2.7.1 Birthday Attacks
> *Birthday Attacks are named after the [birthday paradox](https://en.wikipedia.org/wiki/Birthday_problem)*

- An attack that depends on the fact that *collisions* (duplicate values) appear much faster than one would expect.
- Simple example:
	- Fresh 64-bit authentication key for ea txn ($2^{64}$ possible key values.) 
	- After about $2^{32}$ transactions, the attacker can expect a collision (two txns using same key). 
	- Suppose first authenticated message is always the same, attacker can detect duplicate MAC values.
	- Attacker could replace new $m'$ with old $m$
- If an element can take on $N$ (365) values, then first collision should be after about $\sqrt{N}$ (~19) random elements.
- If you choose $k$ elements, then there are $k(k-1)/2$ pairs of elements.
	- ea with a $1/N$ chance of being a pair of equal values. 
	- chance of collision (close to): $k(k-1)/2N$ 

#### 2.7.2 Meet-in-the-Middle Attacks
> Cousin of birthday attacks; together called collision attacks.

- More common and more useful.
- Instead of waiting for a key to repeat (birthday attack), attacker can build a table of keys.
- Simple example:
	- Fresh 64-bit key to auth ea txn.
	- Attacker chooses $2^{32}$ different 64-bit keys at random. 
	- For ea key, attacker computes the MAC for $m$ (first message); stores MAC and key in table.
	- Attacker then eavesdrops on ea txn and checks for the MAC of $m$.
	- If MAC appears in the table, high chance the auth key for that txn is the same as the attacked computed; now attacker knows auth key ($K_a$).
	- Attacker can now insert arbitrary messages into the txn (birthday attack only allowed for inserting old messages.)
	- Suppose $N$ possible values:
		- First set has $P$ elements.
		- Second has $Q$ elements.
		- There are $PQ$ pairs of elements .
		- Each pair has a $1/N$ chance of matching.
		- Collision expected when $PQ/N$ is close to 1 ($P \approx Q \approx \sqrt {N})$ 

### 2.8 Security Level
> With enough effort, any practical cryptographic system can be attacked successfully. 

- An easy way to quantify the workload of an attack is to compare it to an exhaustive search. 
	- *exhaustive search attack*:  an attack that tries all possible values for some target object, like the key (ex. $K_e$)
	- If an attack requires $2^{235}$ steps of work, then this corresponds to an exhaustive search for a 235-bit value
- *step*: A step can be executed by a computer in a very short time. 
	- The ease of using a step-based analysis far outweighs the built-in inaccuracies.
	- Quick estimate: single step ~ single clock- cycle
- Any system designed today needs at least 128-bit security level*.
- When building crypto systems, should aim to build for next 50 years of use. 
- Crypto primitives are often engineered around powers of two.
- This concept of security level is only approx. 
- The level of security is a function of the access of the adversary. 

### 2.9 Performance
> Security does not come for free. 

- Cryptographers try and make crypo algos as efficient as possible, algos still sometimes perceived too slow. 
	- Creating custom crypto for efficiency can be risky. 
	- You have to do an enormous amount of analysis, from experienced cryptographers, to ensure you don't accidentally create a weak system. 
	- Very expensive.
- For most systems the performance of the crypto is not an issue since modern *CPUs* are fast enough to handle most data streams. 
- Even in systems where crypto is a bottleneck, cheaper to but hardware accelerators. 

### 2.10 Complexity
> The more complex a system is, the more likely it has security problems.

- Part of the problem: "test-and-fix" dev, it only works well for the things it was tested for. 
	- Might be good enough for functionality, not for security systems.
	- The system cannot be tested for all possible attacks from clever malicious attacker. 
	- Attacker wins by finding $\geq$ 1 aspect that wasn't tested. 
- Only to make a system simple is to modularize it.
	- This is generally known from software dev, but with crypto systems no bugs afforded. 
- Correctness must be a local property: one part of the system should behave correctly, regardless of how the rest of the system works. 

### 2.11 Exercises
- [Chapter 2 Exercises](https://github.com/doulos819/mjr/blob/main/research/Notes/books/Crytpo%20Eng%20Exercises.md#chapter-2)

## Part 2: Message Security
In this part:
- Chapter 3: Block Ciphers
- Chapter 4: Block Cipher Modes
- Chapter 5: Hash Functions
- Chapter 6: Message Authentication Codes
- The Secure Channel
- Implementation Issues

## Ch 3. Block Ciphers
> Block ciphers are one of the fundamental building blocks for crypto systems. 

## 3.1 What is a Block Cipher?
> An encryption function for fixed-size blocks of data. 

- Current generation of block ciphers*:
	- block size of 128 bits (16 bytes)
	- encrypt 128-bit plaintext and generate 128-bit ciphertext
	- block cipher is reversible; there is a decryption function that takes the 128-bit ciphertext -> plaintext.
	- plaintext and ciphertext are always the same size; block size
- Like the plaintext and ciphertext, the secret key is also a string of bits. 
	- Commonly 128 and 256 bits; written as $E(K,p)$ or $E_k(p)$ for the encryption of plaintext $p$ with key $K$ and $D(K,c)$ or $D_k(c)$.
- Instead of using block cipher directly, one should use a *block cipher mode* (Ch. 4).
- Don't ever trust a secret block cipher.
- Can be useful to look at a block cipher like a very large key-dependant table.
	- For any fixed key, the table would map the plaintext to the ciphertext.
	- Table would be huge; cipher with 32-bit block size, 16 GB table; 64-bit block size, 150 million TB; 128-bit block size, $5 \cdot 10^{39}$ bytes.
	- Block cipher is reversible; no two entries are the same; permutation.
	- Definition: block cipher takes all the $2^k$ possible k-bit inputs and maps each to a unique output. 
	- Example: if $k = 8$, an input of $00000001$ might encrypt to $01000000$ under a given key, but it might also encrypt to $11011110$ under a different key, depending on the design of the block cipher. 
	  
	  ### 3.2 Types of Attack
	  > All attack types from 2.6 pertain to block ciphers, some are specific to them. Generally, a secure block cipher is one that keeps $p$ secret, but that is only sufficient to mitigate against plaintext-only attackers.
	  
	  - Related-key attack (Eli Biham - 1993):
		  - attacker has access to several encryption functions.
		  - These functions all have an unknown keys but share a relationship that the attacker knows (key inc by 1).
	- Chosen-key attack (Authors working on Twofish): attacker specifies some part of the key, then performs related-key attack. 
	- [Davies-Meyer hash function:](https://en.wikipedia.org/wiki/One-way_compression_function#Davies%E2%80%93Meyer) attacker gets to choose the key of the block cipher, which allows related-key and chosen-key attacks (Ch. 5).
	- Challenge: Define the properties that one reasonably expects from a block cipher.

### 3.3 The Ideal Block Cipher
> What the block cipher primitives community believes a block cipher to be.

What would the ideal block cipher look like:
- It should be a random permutation.
- As in 3.1:
	- 128-bit block cipher ~ lookup table of $2^{128}$ elements of 128 bits each.
	- Ideal block cipher consists of one table for each key value, each table chosen randomly from the set of all possible permutations. 
	- Not looking at one cipher, but treat the ideal block cipher as a uniform probability distribution over the set of all possible block ciphers. 
		- Not something that can be obtained in practice; it is an abstract concept that is used when discussing security. 

### 3.4 Definition of Block Cipher Security
> Definition 1: A secure block cipher is one for which no attack exists.
> 
> Definition 2: An attack on a block cipher is a non-generic method of distinguishing the block cipher from an ideal block cipher. 
> 
> Definition 3: An ideal block cipher implements an independently chosen random even permutation for each of the key values. 

What does it mean to distinguish a block cipher from an ideal block cipher. 
- Given a block cipher $X$, we compare it to an ideal block cipher with the same block size and the same key size. 
- A *distinguisher*: is an algo that is given a black-box function that computes the block cipher $X$ or an ideal block cipher.
- The distinguisher algo is free to choose any key for each of the encryptions and decryptions it performs. 
- Task: figure out whether the black-box function implements the block cipher $X$ or the ideal cipher; doesn't have to be perfect, just correct significantly more than incorrect.
- Choosing the plaintext $0$ and the key $0$ is a distinguisher, but to make it an attack, the distinguisher must be non-generic.
- A distinguisher is generic if a similar distinguisher can be found for almost any block cipher. 
- Nobody has been able to formalize a definition of generic attacks and block cipher security. 
- Must be sure to limit the computational power of the distinguisher.
	- an exhaustive search of half the key space requires $2^{n-1}$ work and provides the right answer 75% of the time (either attacker finds key, or guesses with 50% probability).
	- Not classified as an attack since there's a trade off between amount of work and probability. 
- If a secure block cipher is used, it is no longer necessary to remember any particularities or imperfections; the cipher will have all expected properties; makes the design of larger systems easier.

#### 3.4.1 Parity of a Permutation
> It turns out there are two types of permutations: those that can be constructed from an even number of swaps, and those that can be constructed from an odd number of swaps; half of all permutations are even, half are odd.

- Most modern block ciphers* have a 128-bit block size, but they operate on 32-bit words. 
	- Encryption function is built from many 32-bit operations.
	- It is harder to build an odd permutation from small operations; as a result, almost all block ciphers only generate even permutations. 
- *Parity attack*: For a given key, extract permutation by encrypting all possible plaintexts.
	- If odd, we know its an ideal block cipher (most ciphers only produce even permutations).
	- If even, attacker claims to have the real block cipher and is correct 75% of the time. 
	- No practical significance to the attack; attacker would have to compute all but one of the plaintext/ciphertext pairs of the encryption function.
		- Never allow that many plaintext/ciphertext queries to a block cipher in a real system (other types of attacks would hurt much sooner).
		- Once the attacker knows most of the plaintext/ciphertext pairs, they no longer need a key to decrypt messages, but can simply use a lookup table created form those pairs. 
	- It more important to exhibit professional paranoia and consider a superset of realistic attacks, and then pare away the unrealistic ones, than to start with only realistic attacks and try to find new ones. 

### 3.5 Real Block Ciphers
> It is very difficult to create a block cipher what is efficient in a wide variety of different applications.

- The cryptographic community doesn't trust a cipher until it has been reviewed thoroughly by other experts.
- There are so many ciphers out there that few get any effective peer review. 
- Virtually all block ciphers consist of several repetitions of a weak block cipher, known as a *round*. 
	- Several of these weak rounds in sequence make a strong block cipher. 
- Most attacks on block ciphers begin by attacking versions with a reduced number of rounds. 

#### 3.5.1 DES
> Data Encryption Standard: restricted key size of 56 bits & small block size of 64 bits make it unsuitable for today;s fast computers and large amounts of data.

- Survives in the form of 3DES, which is a block cipher build from three DES in sequence.*
	- encrypt with DES with one 56-bit key, decrypt with a second 56-bit key, and then encrypt again either with the first key or a third 56-bit key.
	- Solved immediate problem of small key size, no known fix for small block size.
- Neither DES of 3DES should be used in today's system, but classical design worth studying. 
- ![](Pasted%20image%2020221104105947.png)
- DES has a 64-bit plaintext; two 32-bit halves $L$ and $R$.
- DES has 16 rounds $R_{1...16}$ ; each round $i$ uses a separate 48-bit round key $K_i$.
	- Each round key is formed by selecting 48 bits from the 56-bit key; each round has a different selection method.
	- The algo that derives $Ki$ from the main block cipher key is called the key schedule.
- Expand: produce 48 bits of output from the 32-bit input. 
- 48-bit output is $\oplus$ with 48-bit $K_i$.
- Result is used in the S-box tables.
	- S-bot(*substitution box*): lookup table that is publicly known. 
	- Consist of either small lookup tables, each maps 6 bits to 4 bits; brings result back to 32 bits. 
	- Provide nonliterary, without them, the cipher could be written as a bunch of binary additions. Easy mathematical attack.
- This basic (repeated) structure is called the [Feistel construction](https://en.wikipedia.org/wiki/Feistel_cipher)
- Elegant block cipher, decryption requires exactly the same set of operations as encryption. 
	- Most leave out swap after last round so hardware implementations can use same circuit to compute both encryptions and decryptions. 
- The combination of the S-box, expand, and bit shuffle functions provide diffusion; ensure that if one bit is changed in the input of $F$, more than one bit is changed in the output.
	- more bit changes after each round.
- DES has a number of properties that disqualify it according to security definition. #edit 
	- Distinguishing attack #edit when cipher key is 0 since all round keys would also be 0; ideal block cipher doesn't have such an identifying property. 
	- DES also has a complementation property that ensures that:
		- $E(\overline{K},\overline{P}) = \overline{E(K,P)}$ 
		- $\forall$ keys $K$, and plaintext $P$, where $\overline{X}$ is the value obtained by complimenting all the bots in $X$. 

#### 3.5.2 AES
> Advanced Encryption Standard; U.S gov standard created to replace DES.

- Became a standard in 2001; not a Feistel cipher. 
- ![](Pasted%20image%2020221104114542.png)
- The plaintext $P$ comes in as 16 bytes (128 bits) at the top.
- 1st operation is to $P \oplus K_i$ (16 byte round key).
- Each of the 16 bytes output is used in index into an S-box table that maps 8-bit inputs to 8-bit outputs. (S-boxes are all identical)
	- bytes are then rearranged.
- Bytes are mixed into groups of four using a linear function.
- A full encryption consists of 10-14 rounds, depending on key size. 
	- AES is defined for 128, 192, and 256-bit keys (10, 12, 14 rounds).
- Each step consists of a number of operations that can be performed in parallel. 
	- On the other hand, decryption is different from encryption; need inverse lookup tabled of the S-box, and inverse mixing operation is different.
- Attack at 6 rounds so designers chose 10-14; attacks have improved.
- More recent attack results show that AES does not meet strict definition of security for a block cipher either. (attacks on 192-bit and 256-bit are theoretical)
- AES still used, build for the ability to replace block cipher if needed. 

#### 3.5.3 Serpent
> Another AES finalist, built like a tank, designed for security all the way.

- Best known attack covers 12/32 rounds, but 1/3 the speed of AES.
- 32 Rounds, each round: 
	- 128-bot $K_i \oplus P$
	- Apply linear mixing function to the 128 bits.
	- Apply 32 four-bit S-boxes in parallel.
		- Each round uses eight different S-boxes in turn (same each round).
		- Total of 1024 S-box lookups which is slow, trick is to rewrite the S-boxes as Boolean formula. 
		-  Each of the four output bits is written as a Boolean formula of the four input bits. (CPU evaluate using AND, OR, and XOR)
		- 32-bit CPU can evaluate 32 S-box lookups in parallel; called bitslice implementation.

#### 3.5.4 Twofish
> Can be seen an a compromise between AES and Serpent. Nearly as fast as AES, but larger security margin.

- Best known attack is on 8/16 rounds #edit 
- Biggest disadvantage is that it can be expensive to change the encryption key because there is a lot of precomputation on the keys in Twofish. 
- Uses same Feistel structure as DES.
- ![](Pasted%20image%2020221107160231.png)
- Splits the 128-bit plaintext into four 32-bit values ($k_{0..3}$).
- Feistel structure can be seen with $F$ being the round function. 
	- Round consists of two copies of the $g$ function.
		- Each consists of four S-boxes followed by a linear mixing function (similar to AES mixing).
		- These S-boxes are not constant, but contents depend on the key. (harder for attacker to analyze)
	- A function called the PHT.
		- Mixes the two results of the $g$ functions 
	- And a Key addition.
		- The last part of the $F$ function.
- The result of the $F$ function is xored into the right half,
- The boxes with ≪ or ≫ symbols in them denote rotations of the 32-bit value by the speciﬁed number of bit positions.

#### 3.5.5 Other AES Finalists 
> 3/5 already discussed. RC6 and MARS as well.

### 3.5.6 Which Block Cipher Should I choose?
> Recent attacks make AES tough choice, still recommend AES, it is fast; attacks are theoretical, not practical.

- Relatively easy to use and implement.
- You could always double encrypt (first with AES, then Serpent or Twofish)
	- If this is done, remember to use different independent keys for each block cipher.
- AES could also be used with an increased number of rounds (28 rounds for 256-bit keys).
- No known attacks against the mathematics of AES, but it is possible to implement poorly.
	- Can be implemented so that the time it takes to perform an operation is dependant on inputs. 
	- If an attacker can measure this time, they might be able to learn bits of the key. 
	- Use constant time implementation or somehow otherwise conceal the timing information. 

#### 3.5.7 What Key Size Should I Use?
> For almost all applications, 128-bit security is enough; to achieve 128-bits of security, use a key longer than 128 bits. 

- For a security level of $n$ bits, every cryptographic value should be at least $2n$ bits long. (128-bit security, use block cipher with a 256 bit block size).
- Larger key provides a better safety margin, assuming that the block cipher is secure. 
- AES with 192 and 256-bit keys are not secure; attacks exploit weaknesses in the key schedule algo.
- No known attacks against AES with 128-bit keys #edit.

## Ch 4. Block Cipher Modes
>

## Ch 5. Hash Functions
> A hash function is a function that takes as input an arbitrarily long string of bits (or bytes) and produces a fixed-size result. A typical use of a hash function is digital signatures.

- The result of $h$ is typically between 128 and 1024 bits, compared to multiple thousands or millions of bits for the message $m$ itself.
- Signing $h(m)$ is therefore much faster than signing m directly.
- Many times when you have a variable-sized value, you can use a hash function to map it to a fixed-size value.
- Hash functions can be used in cryptographic pseudorandom number generators to generate several keys from a single shared secret.
- And they have a one-way property

### 5.1 Security of Hash Functions
- collision resistance. A collision is two different inputs m1 and m2 for which h(m1) = h(m2).
- Thus, a hash function is never collision-free. The collision-resistance requirement merely states that, although collisions exist, they cannot be found.
- In practice, cryptographic designers expect a hash function to be a random mapping. Therefore, we require that a hash function be indistinguishable from a random mapping.
- ** Definition 4** The ideal hash function behaves like a random mapping from all possible input values to the set of all possible output values.
- **Definition 5** An attack on a hash function is a non-generic method of distinguishing the hash function from an ideal hash function.
- Unlike the block cipher, the hash function has no key, and there is no generic attack like the exhaustive key search.
- One generic attack on a hash function is the birthday attack, which generates collisions. For a hash function with an n-bit output, this requires about 2n/2 steps.
- In other situations, the goal is to find a pre-image (given x, find an m with h(m) = x), or to find some kind of structure in the hash outputs. The generic pre-image attack requires about 2n steps.
- As with block ciphers, we allow a reduced security level if it is specified. We can imagine a 512-bit hash function that specifies a security level of 128 bits. In that case, distinguishers are limited to 2128 steps.

### 5.2 Real Hash Functions
- Almost all real-life hash functions, and all the ones we will discuss, are iterative hash functions.
- Iterative hash functions split the input into a sequence of fixed-size blocks m1, ... , mk, using a padding rule to fill out the last block.
- A typical block length is 512 bits, and the last block will typically contain a string representing the length of the input.
- The message blocks are then processed in order, using a compression function and a fixed-size intermediate state. This process starts with a fixed value H0, and defines Hi = h (Hi−1, mi). 
- The final value Hk is the result of the hash function.

### 5.2.2 MD5
- MD5 has now been broken too. You will still hear people talk about MD5, however, and it is still in use in some real systems.
- MD5 has a 128-bit state that is split into four words of 32 bits each.
- This structure of operating on 32-bit words is very efficient on 32-bit CPUs.
- For most applications, the 128-bit hash size of MD5 is insufficient. 
	- Using the birthday paradox, we can trivially find collisions on any 128-bit hash function using 264 evaluations of the hash function.
- MD5’s internal structure makes it vulnerable to more efficient attacks.
- But recent cryptanalytic advances, beginning with Wang and Yu [124], have now shown that it is actually possible to find collisions for the full MD5 using much fewer than 264 MD5 computations.

#### 5.2.3 SHA-1
- Independent of any internal weaknesses, the main problem with SHA-1 is the 160-bit result size. Collisions against any 160-bit hash function can be generated in only 280 steps, well below the security level of modern block ciphers with key sizes from 128 to 256 bits.
- Although it took longer for SHA-1 to fall than MD5, we now know that it is possible to find collisions in SHA-1 using much less work than 280 SHA-1 computations.

#### 5.2.4 SHA-224, SHA-256, SHA-384, and SHA-512
- These hash functions are collectively referred to as the SHA-2 family of hash functions.
- 
- SHA-256 is much slower than SHA-1. For long messages, computing a hash with SHA-256 takes about as much time as encrypting the message with AES or Twofish, or maybe a little bit more. This is not necessarily bad, and is an artifact of its conservative design.

### 5.3 Weaknesses of Hash Functions
- Our greatest concern about all these hash functions is that they have a length extension bug that leads to real problems and that could easily have been avoided.
- The length extension problem exists because there is no special processing at the end of the hash function computation. The result is that $h(m)$ provides direct information about the intermediate state after the first $k$ blocks of $m'$.
- This property immediately disqualifies all of the mentioned hash functions, according to our security definition. All a distinguisher has to do is to construct a few suitable pairs $(m, m')$ and check for this relationship. You certainly wouldn’t find this relationship in an ideal hash function.

#### 5.3.2 Partial-Message Collision
- A second problem is inherent in the iterative structure of most hash functions. We’ll explain the problem with a specific distinguisher.
- For a perfect hash function of size n, we expect that this construction has a security level of n bits. The attacker cannot do any better than to choose an m, get the system to authenticate it as h(m || X), and then search for X by exhaustive search.
- The attacker can do much better with an iterative hash function.
- The attacker can do much better with an iterative hash function. She finds two strings $m$ and $m'$ that lead to a collision when hashed by h. This can be done using the birthday attack in only 2n/2 steps or so.

### 5.4 Fixing the Weaknesses
- Leaving weaknesses in the hash function is a very bad idea.

#### 5.4.1 Toward a Short-term Fix
- **Definition 6** Let h be an iterative hash function. The hash function $h_{DBL}$ is defined by $h_{DBL}(m) := h(h(m) || m)$.
- We believe that if h is any of the newer SHA-2 family hash functions, this construction has a security level of n bits, where n is the size of the hash result. A disadvantage of this approach is that it is slow.

#### 5.4.2 A More Efficient Short-term Fix
- **Definition 7** Let h be an iterative hash function, and let b denote the block length of the underlying compression function. The hash function hd is defined by $h_d(m) := h(h(0^b || m))$, and has a claimed security level of min(k, n/2) where k is the security level of h and n is the size of the hash result.
   Here b is the block length of the underlying compression function, so 0^b || m equates to prepending the message with an all zero block before hashing.
- We will use this construction mostly in combination with hash functions from the SHA family.
- SHAd-256 is just the function $m \rightarrow SHA-256(SHA-256(0^{512} || m))$, for example.
- Whether $h_{DBL}$ in fact has a security level of n bits remains to be seen. We would trust both of them up to n/2 bits of security, so in practice we would use the more efficient h_d construction.

#### 5.4.3 Another Fix
- There is another fix to some of these weaknesses with the SHA-2 family of iterative hash functions: Truncate the output. If a hash function produces n-bit outputs, only use the first n − s of those bits as the hash value for some positive s.
- For 128 bits of security, you could hash with SHA-512, drop 256 bits of the output, and return the remaining 256 bits as the result of the truncated hash function. The result would be a 256-bit hash function which, because of birthday attacks, would meet our 128-bit security design goal.

### 5.5 Which Hash Function Should I Choose?
- SHA-3

## Ch. 6 Message Authentication Codes
> A message authentication code, or MAC, is a construction that detects tampering with messages. Encryption prevents Eve from reading the messages but does not prevent her from manipulating the messages. This is where the MAC comes in.

### 6.1 What a MAC Does
-  We’ll write the MAC function as mac(K, m). To authenticate a message, Alice sends not only the message m but also the MAC code mac(K, m), also called the tag.
- Bob uses the MAC verification algorithm to verify that T is a valid MAC under key K for message $m'$.

### 6.2 The Ideal MAC and MAC Security
- The primary difference is that block ciphers are permutations, whereas MACs are not.
- The ideal MAC is a random mapping. Let n be the number of bits in the result of a MAC. Our definition of an ideal MAC is:
	- Definition 8 An ideal MAC function is a random mapping from all possible inputs to n-bit outputs.
	- Definition 9 An attack on a MAC is a non-generic method of distinguishing the MAC from an ideal MAC function.
- The more restrictive standard definition is one in which an attacker selects n different messages of her choosing, and is given the MAC value for each of these messages. The attacker then has to come up with n + 1 messages, each with a valid MAC value.

### 6.3 CBC-MAC and CMAC**
- CBC-MAC is a classic method of turning a block cipher into a MAC.
- The idea behind CBC-MAC is to encrypt the message m using CBC mode and then throw away all but the last block of ciphertext.
- For a message P1, ... , Pk, the MAC is computed as: 
	- $H_0 := \text{IV}$
	- $H_i := E_K(P_i ⊕ H_{i−1})$ 
	- $\text{MAC} := H_k$
- In general, one should never use the same key for both encryption and authentication.
- There are a number of different collision attacks on CBC-MAC that effectively limit the security to half the length of the block size [20].
- Here is a simple collision attack:
	- let M be a CBC-MAC function.
	- If we know that $M(a) = M(b)$ then we also know that $M(a || c) = M(b || c)$.
		- This is due to the structure of CBC-MAC.
	- $M(a || c) = E_K(c \oplus M(a))$ 
	- $M(b || c) = E_K(c \oplus M(b))$ 
		- and these two must be equal, because $M(a) = M(b)$.
	- Because of birthday attack, this takes $2^{64}$ steps for a 128-bit block cipher
	- If you wish to use CBC-MAC, you should instead do the following:
		1. Construct a string $s$ from the concatenation of $l$ and $m$, where $l$ is the length of $m$ encoded in a fixed-length format. 
		2. Pad $s$ until the length is a multiple of the block size. (See Section 4.1 for details.) 
		3. Apply CBC-MAC to the padded string $s$. 
		4. Output the last ciphertext block, or part of that block. Do not output any of the intermediate values.
- CMAC works almost exactly like CBC-MAC, except it treats the last block differently. Specifically, CMAC xors one of two special values into the last block prior to the last block cipher encryption.
	- The xoring of these values into the MAC disrupts the attacks that compromise CBC-MAC when used for messages of multiple lengths.

### 6.4 HMAC
- HMAC computes h(K ⊕ a || h(K ⊕ b || m)), where a and b are specified constants.
- The message itself is only hashed once, and the output is hashed again with the key.
- To achieve our 128-bit security level, we would only use it with a 256-bit hash function such as SHA-256.

### 6.5 GMAC
- The GMAC authentication function takes three values as input— the key, the message to authenticate, and a nonce.
- Unforgeability definition: 
- Under the hood, GMAC uses something called an universal hash function
- consider a model in which an attacker selects n different messages of his choosing, and is given the MAC value for each of these messages:
	- The attacker then has to come up with n + 1 messages, each with a valid MAC value. 
	- If an attacker can’t do this, then the MAC is unforgeable.
- GMAC computes a simple mathematical function of the input message.
- GMAC then encrypts the output of that function with a block cipher in CTR mode to get the tag.
- Requiring the system to provide a nonce can be risky because security can be undone if the system provides the same value for the nonce more than once.

### 6.6 Which MAC to Choose?**
- HMAC - As far as we know, there is no collision attack on the MAC value if it is used in the traditional manner, so truncating the results from HMAC-SHA-256 to 128 bits should be safe, given current knowledge in the field.
- GMAC is fast, but provides only at most 64 bits of security and isn’t suitable when used to produce short tags.

### 6.7 Using a MAC
- The Horton Principle: Authenticate what is meant, not what is said.
- You should authenticate the meaning, not the message. This means that the MAC should authenticate not only m, but also all the information that Bob uses in parsing m into its meaning.
- data like protocol identifier, protocol version number, protocol message identifier, sizes for various fields, etc. One partial solution is to not just concatenate the fields but use a data structure like XML that can be parsed without further information.
  >To recap: whenever you do authentication, always think carefully about what other information should be included in the authentication. Be sure that you code all of this information, including the message, into a string of bytes in a way that can be parsed back into the fields in a unique manner. Do not forget to apply this to the concatenation of the additional data and the message we discussed at the start of this section. If you authenticate d || m, you had better have a fixed rule on how to split the concatenation back into d and m.
  
  ## Ch. 7
- [week-4](week-4.md)

## Ch. 8 Implementation Issues
> Easy to screw up the security at the implementation level. In fact, implementation errors such as buffer overflows are one of the biggest security problems in real-world systems.

- impossible to implement a secure system
	- some systems are not designed for security, but we do our best to make sure at least our part is secure.
	- We care mostly about our own part of the system because it is all we can control.
- Attacks on crypto can be invisible and attacks on crypto are unlikely to be noticed.
- Long term goal is secure computer systems.
	- Try our best to make sure weakest link is not the crypto, easy to switch crypto systems after implemented. 
- Changing communication protocols can take a while and they can be in use for 30-50 years after implementation. 

#### 8.1.1 Specs
> If there are no specifications, then you cannot even check whether a program is correct or not. For such programs, the whole concept of correctness is undefined.

- functional specification: in practice... this document often does not exist, is incomplete, or specifies things that are irrelevant for the behavior of the program.
- There are really three stages in the specification process::
	1. Requirements are an informal description of what the program is supposed to achieve. I
	2. The functional specification gives a detailed and exhaustive definition of the behavior of the program. 
		- Anything not in the functional specification does not have to be implemented. Any item can, and should, be tested.
	3. Implementation design This document has many names, but it specifies how the program works internally. It contains all of the things that cannot be tested from the outside.
- Of these three documents, the functional specification is without a doubt the most important one.
- Without functional specifications, there is no way to even describe what you have achieved in the end when the program is finished.without functional specifications, there is no way to even describe what you have achieved in the end when the program is finished.

#### 8.1.2 Test and Fix
> The second problem in writing correct programs is the test-and-fix development method that is in almost universal use. Programmers write a program, and then test whether it behaves correctly.

- If it doesn’t, they fix the bugs and test again. As we all know, this does not lead to a correct program. It results in a program that kind of works in the most common situations.
- There are some simple rules about bugs that any good software engineering book includes:
	- If you find a bug, first implement a test that detects the bug. Check that the bug is detected. Then fix the bug, and check that the test no longer finds the bug. And then keep running that test on every future version to make sure the bug does not reappear. - -
	- Whenever you find a bug, think about what caused it. Are there any other places in the program where a similar bug might reside? Go check them all. 
	- Keep track of every bug you find. Simple statistical analysis of the bugs you have found can show you which part of the program is especially buggy, or what type of error is made most frequently, etc. Such feedback is necessary for a quality control system.

#### 8.1.3 Lax Attitude
> The third problem is the incredibly lax attitude of many in the computer industry. Errors in programs are frequently accepted as a matter of course.

- ’ Software companies routinely ship products with known bugs in them. This wouldn’t be so bad if they only sold computer games, but nowadays our work, our economy, and—more and more—our lives depend on software.
- If a car manufacturer finds a defect (bug) in a car after it was sold, they will recall the car and fix it. 
- Software companies get away with disclaiming any and all liability in their software license, something they wouldn’t be allowed to do if they produced any other product. 
- This lax attitude means there are still not enough serious efforts being made at producing correct software.

#### 8.1.4 So How Do We Proceed?
> Don’t ever think that all you need is a good programmer or code reviews or an ISO 9001–certified development process or extensive testing or even a combination of all of them. Reality is much more difficult.

- The airline industry has been amazingly effective at making flying secure. We would do well to learn all we can from them. Maybe writing correct software would cost an order of magnitude more than what we are used to now. But given the cost to society of the bugs in software that we see today, we are sure it would be cost-effective in the long run.

### 8.2 Creating Secure Software
> Just writing correct software is not good enough for a security system. The software must be secure as well. What is the difference?

- Correct software has a specified functionality. If you hit button A, then B will happen. Secure software has an additional requirement: a lack of functionality. 
	- No matter what the attacker does, she cannot do X. This is a very fundamental difference; you can test for functionality, but not for lack of functionality.
- The inevitable conclusion is: Standard implementation techniques are entirely inadequate to create secure code.
- Let us make our point of view clear: unless you are willing to put real effort into developing a secure implementation, there is little point in bothering with the cryptography. Designing cryptographic systems might be fun, but cryptography is generally only a small part of a larger system.

### 8.3 Keeping Secrets
> from this blog: https://benma.github.io/2020/10/16/rust-zeroize-move.html

### A pitfall of Rust's move/copy/drop semantics and zeroing data
> oct 16, 2020: We are using Rust extensively in the firmware of the [BitBox02](https://shiftcrypto.ch/bitbox02/) hardware wallet.

- In a security device like this, you don’t want to leave sensitive material in memory for longer than necessary.
-  In particular, when the value is being dropped, the memory should be safely overwritten with zeroes, to mitigate the risks of the memory leaking.
- [zeroize](https://docs.rs/zeroize/1.1.1/zeroize/) is a crate designed to make this task easy and safe.
	-  it allows you to wrap your types in `zeroize::Zeroize<>`, so the value will be automatically zeroed on drop.
- It turns out it is still quite easy to accidentally leave copies of the sensitive data in memory.
```rust
use zeroize::Zeroize;

#[derive(Debug)]
struct EncryptionKey([u8; 4]);

impl Drop for EncryptionKey {
    fn drop(&mut self) {
        println!("Pointer when zeroing: {:p}", self.0.as_ptr());
        self.0.zeroize();
        println!("Zeroed. Remaining value: {:?}", self.0);
    }
}

fn get_encryption_key() -> EncryptionKey {
    let key = EncryptionKey(*b"AKey");
    println!("Pointer at creation: {:p}", key.0.as_ptr());
    key
}

fn main() {
    let encryption_key = get_encryption_key();
    let ptr = encryption_key.0.as_ptr();

    println!("Pointer when using: {:p}", encryption_key.0.as_ptr());
    println!("Using key to encrypt stuff: {:?}", encryption_key);

    println!("About to drop.");
    drop(encryption_key);
    println!("Dropped.");

    println!("Memory: {:?}", unsafe {
        core::slice::from_raw_parts(ptr, 4)
    });
```

}
- Output: 
```rust
Pointer at creation: 0x7ffd632b0ba8
Pointer when using: 0x7ffd632b0c90
Using key to encrypt stuff: EncryptionKey([65, 75, 101, 121])
About to drop.
Pointer when zeroing: 0x7ffd632b0c10
Zeroed. Remaining value: [0, 0, 0, 0]
Dropped.
Memory: [65, 75, 101, 121]
```
- in Rust, **moving** a value compiles into a **memory copy** in the general case.
	- For example, the `key` var in `get_encryption_key()` is a local stack variable, so returning it (moving it out of the function) must be a memory copy under the hood.
	- What about the manual `drop()`? Same thing: the value is _moved_ into the `drop()` function, but the value is copied in memory while doing so.
- To fix this, stack vars can be allocated at the top and pushed down into functions as mutable arguments, but that leads to hard to understand and hard to maintain code. 

It is much easier to use the heap from the start, where the location is permanent (_caveats apply here too, as described by the zeroize docs!_).

What is copied is not the underlying data, but just the `Box` metadata:

```rust
use zeroize::Zeroize;

#[derive(Debug)]
struct EncryptionKey(Box<[u8; 4]>);

impl Drop for EncryptionKey {
    fn drop(&mut self) {
        println!("Pointer when zeroing: {:p}", self.0.as_ptr());
        self.0.zeroize();
        println!("Zeroed. Remaining value: {:?}", self.0);
    }
}

fn get_encryption_key() -> EncryptionKey {
    let key = EncryptionKey(Box::new(*b"AKey"));
    println!("Pointer at creation: {:p}", key.0.as_ptr());
    key
}

fn main() {
    let encryption_key = get_encryption_key();
    let ptr = encryption_key.0.as_ptr();
    
    println!("Pointer when using: {:p}", encryption_key.0.as_ptr());
    println!("Using key to encrypt stuff: {:?}", encryption_key);

    println!("About to drop.");
    drop(encryption_key);
    println!("Dropped.");

    println!("Memory: {:?}", unsafe {
        core::slice::from_raw_parts(ptr, 4)
    });
}

```

Output:

```rust
Pointer at creation: 0x558449695b40
Pointer when using: 0x558449695b40
Using key to encrypt stuff: EncryptionKey([65, 75, 101, 121])
About to drop.
Pointer when zeroing: 0x558449695b40
Zeroed. Remaining value: [0, 0, 0, 0]
Dropped.
Memory: [0, 0, 0, 0]
```

>The rest of 8.3...
>Anytime you work with cryptography, you are dealing with secrets. And secrets have to be kept. This means that the software that deals with the secrets has to ensure that they don’t leak out

- For the secure channel we have two types of secrets: the keys and the data.
	- Both of these secrets are transient secrets; we don’t have to store them for a long time.
		- The data is only stored while we process each message. 
		- The keys are only stored for the duration of the secure channel.
- Transient secrets are kept in memory. Unfortunately, the memory on most computers is not very secure.

#### 8.3.1 Wiping State
> A basic rule of writing security software: wipe any information as soon as you no longer need it. The longer you keep it, the higher the chance that someone will be able to access it.

- If you write a library for others to use, you have to depend on the main program to inform you that the state is no longer needed. 
	- For example, when the communication connection is closed, the crypto library should be informed so that it can wipe the secure channel session state. 
		- The library can contain a function for this, but there’s a reasonable chance that the programmer of the application won’t call this function. After all, the program works perfectly well without calling this function
- . A typical security-relevant function performs some computations in local variables, and then tries to wipe them.
- It is not uncommon to see code that reveals data that it happens to find in memory.
	- If the memory is given to some library without having been wiped first, the library might leak the data to an attacker.
	- So check the code that your compiler produces, and make sure the secrets are actually being wiped.
- There are other places where secret data can end up. All data is eventually loaded into a CPU register.
	- Wiping registers is not possible in most programming languages, but on register-starved CPUs like the x86, it is very unlikely that any data will survive for any reasonable amount of time.
- During a context-switch (when the operating system switches from running one program to running the next program), the values in the registers of the CPU are stored in memory where their values might linger for a long time. 
	- As far as we know, there is nothing you can do about this, apart from fixing the operating system to ensure the confidentiality of that data.

#### 8.3.2 Swap File
> Most operating systems (including all current Windows versions and all UNIX versions) use a virtual memory system to increase the number of programs that can be run in parallel. While a program is running, not all of its data is kept in memory. Some is stored in a swap file. When the program tries to access data that is not in memory, the program is interrupted.

- The virtual memory system reads the required data from the swap file into a piece of memory, and the program is allowed to continue. 
	- What’s more, when the virtual memory system decides that it needs more free memory, it will take an arbitrary piece of memory from a program and write it to the swap file.
- Most software is designed for a cooperative environment, not the adversarial environment that cryptographers work in. 
	- So our problem is the following: 
		- the virtual memory system could just take some of the memory of our program and write it to the swap file on disk. The program never gets told, and does not notice. Suppose this happens to the memory in which the keys are stored. If the computer crashes—or is switched off— the data remains on the disk.
		- There may be no mechanism to wipe the swap file, so the data could linger indefinitely on disk. Who knows who will have access to this swap file in future? We really cannot afford the risk of having our secrets written to the swap file.
	- So how do we stop the virtual memory system from writing our data to disk? 
		- On some operating systems there are system calls that you can use to inform the virtual memory system that specified parts of memory are not to be swapped out. (interesting to look into for Linux)
		- Some operating systems support a secure swap system where the swapped-out data is cryptographically protected.
	- Assuming you can lock the memory and prevent it from being swapped out, which memory should be locked? 
		- All the memory that can ever hold secrets, of course.
	- This brings up a secondary problem. 
		- Many programming environments make it very hard to know where exactly your data is being stored.
		- Probably the best solution is to simply lock all the memory of your application.
			- Even that is not quite as easy as it sounds, because you could lose a number of operating system services such as the automatically allocated stack. And locking all the memory makes the virtual memory system ineffective.

#### 8.3.3 Cashes
> Modern computers don’t just have a single type of memory. They have a hierarchy of memories.

- At the bottom is the main memory—often gigabytes in size. But because the main memory is relatively slow, 
	- There is also a cache. This is a smaller but faster memory.
		- The cache keeps a copy of the most recently used data from the main memory. 
		- If the CPU wants to access the data, it first checks the cache. 
			- If the data is in the cache, the CPU gets the data relatively quickly.
			- If the data is not in the cache, it is read (relatively slowly) from main memory, and a copy is stored in the cache for future use. To make room in the cache, a copy of some other piece of data is thrown away.
		- This is important because caches keep copies of data, including copies of our secret data.
- Manufacturers never specify how to wipe data in a guaranteed manner. At least, we have never seen any specifications like that, and as long as it is not specified, we can’t trust it.

#### 8.3.4 Data Retention by Memory
> Something that surprises many people is that simply overwriting data in memory does not delete the data. 

- If you store data in a memory location, that location slowly starts to ‘‘learn’’ the data. When you overwrite or switch off the computer, the old value is not completely lost.
	- Depending on the circumstances, just powering the memory off and back on again can recover some or all of the old data.
- Similar processes happen in DRAM (Dynamic RAM), although they are somewhat more complicated. 
	- DRAM works by storing a small charge on a very small capacitor. 
	- The insulating material around the capacitor is stressed by the resulting field. 
	- The stress results in changes to the material, specifically causing the migration of impurities [57]. 
		- An attacker with physical control over the memory can potentially recover this data. 
			- Additionally, because of how DRAM capacitors discharge, their values may remain for seconds at room temperature if power is removed or even longer if the memory is cooled.
- If your computer is ever compromised (e.g., stolen), you do not want the data that you had in memory to be compromised as well. To achieve this goal, we have to make the computer forget information.
	- We can only give a partial solution, which works if we make some reasonable assumptions about the memory. This solution, which we call a Boojum, works for relatively small amounts of data, such as keys.
	- For Boojum: 
		- let $m$ be the data we want to store. 
		- Instead of storing $m$, we generate a random string $R$ and store both $R$ and $h(R) ⊕ m$ where $h$ is a hash function. (These two values are stored in different memory locations, preferably not too close together)
		- One trick is to change R regularly. 
			- At regular intervals, say every 1 second, we generate a new random $R'$ , and update the memory to store $R ⊕ R' and h(R ⊕ R ) ⊕ m$.
				- This ensures that each bit of the memory is written with a sequence of random bits. To wipe the memory, you simply write a new $m$ with the value zero.
		- To read information from this storage, you read both parts, hash the first, and xor them together to get $m$. Writing is done by $\text{xoring}$ the new data with $h(R)$ and storing it in the second location.
			- Care should be taken that the bits of $R$ and $h(R) ⊕ m$ are not adjacent on the RAM chip.
			- An even better solution might be to choose two random addresses in a very large address space.
				- This makes the probability that the two locations are adjacent very small, independent of the actual chip layouts of the memory.
		- There is still no guarantee that the memory will be wiped. 
			- If you read the documentation of a memory chip, there are no specifications that prevent the chip from retaining all data ever stored in it. 
				- No chip does that, of course, but it shows that we can at most achieve a heuristic security.
		- If you have large amounts of data that need to be kept secret, then the solution of storing two copies and $\text{xoring}$ new random strings into both copies regularly becomes too expensive.
			- A better solution is to encrypt a large block of data and store the ciphertext in memory that potentially retains information. Only the key needs to be stored in a way that avoids data retention, for example, using a Boojum.

#### 8.3.5 Access by Others
> There’s yet another problem with keeping secrets on a computer: other programs on the same machine might access the data.

- Some operating systems allow different programs to share memory. If the other program can read your secret keys, you have a serious problem.
- Under UNIX, it is sometimes possible to force a core-dump of a program. 
	- The core-dump is a file that contains a memory image of the program data, including all of your secrets.
- Another danger comes from especially powerful users. Called superusers, or administrators, these users can access things on the machine that normal users cannot. Under UNIX, for example, the superuser can read any part of the memory.
- In general, your program cannot effectively defend itself against these types of attacks. If you are careful, you may be able to eliminate some of these problems, but often you’ll find yourself limited in what can be achieved. Still, you should consider these issues on the particular platform you are working on.

#### 8.3.6 Data Integrity
> In addition to keeping secrets, we should protect the integrity of the data we are storing. We use the MAC to protect the integrity of the data during transit, but if the data can be modified in memory, we still have problems.

- If you are unsure about the hardware reliability, perhaps you should spend part of your time and memory simply to verify it, although that is really the operating system’s job.
- One thing we try to do is make sure the main memory on our machines is ECC (error-correcting code) memory. If there is a single bit failure, then the error-correcting code will detect and correct the error. 
	- Without ECC memory, any bit error leads to the CPU reading the wrong data.
	- r. Suppose the engineering is done really well, and each bit has only a $10^{−15}$ chance of failing in each second. If you have 128 MB of memory, then you have about $10^9$ bits of memory, and you can expect one bit failure every 11 days. 
		- The error rate increases with the amount of memory in the machine, so it is even worse if you have 1 GB of memory, with one failure every 32 hours.
- Some of the dangers that threaten data confidentiality also endanger the data integrity. Debuggers can sometimes modify your program’s memory. Superusers can directly modify memory, too. Again, there is nothing you can do about it, but it is useful to be aware of the situation.

#### 8.3.7 What to Do
> Keeping a secret on a modern computer is not as easy as it sounds. There are many ways in which the secret can leak out. To be fully effective, you have to stop all of them.

- You have to do the best you can. This involves a lot of work, all of it specific to the environment you work in.
- These problems also make it very difficult to create a library with the cryptographic functions in it. Keeping the secrets safe often involves modifications to the main program. And of course, the main program also handles data that should be kept confidential; otherwise, it wouldn’t need the cryptography library in the first place. This is the familiar issue of security considerations affecting every part of the system. (confusing)

### 8.4 Quality of Code
> If you create an implementation for a cryptographic system, you will have to spend a great deal of time on the quality of the code.

#### 8.4.1 Simplicity
> Complexity is the main enemy of security. Therefore, any security design should strive for simplicity.

- Eliminate all the options that you can.
- Get rid of all those baroque features that few people use.
- "People always ask for these features, but in many cases they do not realize the consequences of using partial security features. Most users are not informed enough about security to be able to select the correct security options."
	- The best solution is to have no options and make the system secure by default. If you absolutely have to, provide a single option: secure or insecure.
- Many systems also have multiple cipher suites, where the user (or someone else) can choose which cipher and which authentication function to use. If at all possible, eliminate this complexity.
	- After all, if choosing an encryption and authentication mode is so difficult that the designer can’t do it, it will be even more challenging for a user to make an informed decision.

#### 8.4.2 Modularization
> Even after you have eliminated a lot of options and features, the resulting system will still be quite complex. With modularization, you divide the system into separate modules, and design, analyze, and implement each module separately.

- Look closely at the interface of your modules. Often there are features or options that exist to solve some other module’s problems. If possible, rip them out.
	- Each module should solve its own problems.
- Modularization is so important because it is the only efficient way we have of dealing with complexity.
- If you have 20 modules, each with a single binary option that changes the module behavior, there are over a million possible configurations. You would have to analyze each of these configurations for security—an impossible task.
- Many systems contain so-called optimizations that are useless, counterproductive, or insignificant because they do not optimize those parts of the system that form the bottleneck.
- Make sure that work can be done in large enough chunks. Then only optimize those parts of your program that you can measure as having a significant effect on the performance.

#### 8.4.3 Assertions
> Assertions are a good tool to help improve the quality of your code. When implementing cryptographic code, adopt an attitude of professional paranoia.

- Each module distrusts the other modules, and always checks parameter validity, enforces calling sequence restrictions, and refuses unsafe operations. 
	- Most of the times these are straightforward assertions. If the module specifications state that you have to initialize the object before you use it, then using an object before initialization will result in an assertion error.
	- Assertion failures should always lead to an abort of the program with ample documentation of which assertion failed, and for what reason.
		- The general rule is: any time you can make a meaningful check on the internal consistency of the system, you should add an assertion.
		- There are some programmers who implement assertion checking in development, but switch it off when they ship the product. This is not the security perspective.
			- For production Rust, assertions are left out!
- Generating wrong answers is probably the worst thing a program can do. It is much better to at least inform the user that a programming error has occurred, so he does not trust the erroneous results of the program. Our recommendation is to leave all your error checking on. (Rust = tests!)

#### 8.4.4 Buffer Overflows
> Buffer overflow problems have been known for decades. Perfectly good solutions to avoid them have been available for the same amount of time

- Avoid any programming language that allows buffer overflows. Specifically: don’t use C or C++. And don’t ever switch off the array bounds checking of whichever language you use instead. It is such a simple rule, and will probably solve half of all your security bugs.

#### 8.4.5 Testing

> Extensive testing is always part of any good development process. Testing can help find bugs in programs, but it is useless to find security holes. Never confuse testing with security analysis. The two are complementary, but different.

- There are two types of tests that should be implemented.
	1. The first is a generic set of tests developed from the module’s functional specifications. 
		- Ideally, one programmer implements the module and a second programmer implements the tests. Both work from the functional specification. 
			- Any misunderstanding between the two is a clear indication that the specifications have to be clarified. The generic tests should attempt to cover the entire operational spectrum of the module
			- In much of our own code, the test code is about as big as the operational code, and we have not found a way of significantly improving that.
		- A second set of tests is developed by the programmer of the module itself. These are designed to test any implementation limits.
	2. Also useful to have some ‘‘quick test’’ code that can run every time the program starts up.

###  Side-Channel Attacks
> There is a whole class of attacks that we call side-channel attacks [72]. These are possible when an attacker has an additional channel of information about the system. For example, an attacker could make detailed measurements of the time it takes to encrypt a message.

- If the cryptography is embedded in a smart card, then the attacker can measure how much current the card draws over time. Magnetic fields, RF emissions, power consumption, timing, and interference on other data channels can all be used for side-channel attacks.
	- Not surprisingly, side-channel attacks can be remarkably successful against systems that are not designed with these attacks in mind. Power analysis of smart cards is extremely successful [77].
- Resistance against side-channel attacks will always come from a combination of countermeasures—some of them in the software that implements the cryptographic system, and some of them in the actual hardware.
- Preventing side-channel attacks is an arms race. You try to protect yourself against the known side channels, and then a smart person somewhere discovers a new side channel, so then you have to go back and take that one into account as well. 
	- In real life, the situation is not that bad, because most side-channel attacks are difficult to perform.
- Side channels are a real danger to smart cards because the card is under full control of the adversary, but only a few types of side channels are practical against most other computers. 
	- In practice, the most important side channels are timing and RF emissions. (Smart cards are particularly vulnerable to measuring the power consumption.)

### 8.6 Beyond this Chapter
> We hope this chapter has made it clear that security does not start or stop with the cryptographic design. All aspects of the system have to do their part to achieve security.

- Implementing cryptographic systems is an art in itself. The most important aspect is the quality of the code. Low-quality code is the most common cause of real-world attacks, and it is rather easy to avoid. I
- Be fanatical about the quality of your code. It can be done, and it needs to be done, so go do it!

# Part 3: Key Negotiation

## Ch 9. Generating Randomness
> 