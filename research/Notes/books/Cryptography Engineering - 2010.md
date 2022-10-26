# Notes on the Text

###### tags: `cryptography` `uncloak`
Author(s): Niels Ferguson, Bruce Schneier, and Tadayoshi Kohono

Paper(s): 
- [Link to Text](https://drive.google.com/drive/folders/1506sz7G5o6ATeGObP1AEwMV4msaLK3HD?usp=sharing)

Link to exercises: https://github.com/doulos819/mjr/blob/main/research/Notes/books/Crytpo%20Eng%20Exercises.md

### Table of Contents
[toc]

:::info
>Abstract: Most books cover what cryptography is, but this book provides invaluable resources for anyone working with cryptography. Cryptography and security engineers need to know more than just how protocols work; they need to know how to USE CRYPTOGRAPHY! 
>
>To know how to use crytography, one must learn how to think like a crytographer. 
>
>Both computer security and cryptography are about designing and evaluating objects (systems or algorithms) intended to behave in certain ways even in the presence of an adversary. 
>
>Cryptography on world stage (1990s) to secure internet; still no effective way for individuals to leverave. 
:::

## 1. The Contect of Cryptography 
> Cryptography is the art and science of encryption, although also covers much broader set of elementary security functions. This book is designed to help cultivate mentality for thinking about security probelms & a scientific backround. 

First Lesson: Keep a critical mind; don't blindly trust anything.

### 1.1 The Role of Cryptogtaphy
> Cryptography by itself is fairly useless 

- Needs to be part of a larger system
- Similar to lock in real world; if rest of system is insecure, rest of system wil get attacked before lock
- If attack on system, probably noticable; attack on cryptography, probably not noticable (re-entry possible/likely)

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
> It might be part of the solution, or it might be par tof the problem. 

Cryptography can also make systems weaker when use din inapproriate ways. In many situations, it provides only a feeling of security, but no actual security.

### 1.7 Crytography Is Very Difficult
> The weakest-link property and the adversarial setting conspire to make life for a cryptographer-or any security engineer-very hard.

There are some small areas of cryptography that are generally well understood, but that doesnt mean they are simple. These areas have just been worked on for a few decades and general consensus is that critical issues are known.

### 1.8 Cryptography Is The Easy Part
> Like a lock, a cryptographic component has fairly well-defined boundaries and requirements. 

Cryptography is the easy part, because there are people who know how to do a reasonably good job. 
- key management and key storage is crucial to any cryptographic system yet most computers have no secure place to store a key
- mobile devices...

### 1.9 Generic Attacks
> Some security problems can't be solved. 

- It is important to identidy what the generic attacks against a system are
- generic attack from outside the system cannot be protected against (by any technology).

### 1.10 Security And Other Design Criteria
> Security is neevr the only design criterion for a system. Instead security is but one of many criteria.

#### 1.10.1 Security Versus 
> Good engineering lasts the tests of time.

- The priorities of computer industry largely value speed > security
- The result is generally a system that is somewhat efficient, yet is not sufficienly secure.
- "If it fails, fall back off and become more conservative"
- Lack of compiter security is a hinderance to most users.
- Digital crooks of the future will have more resources and the probelm will be larger
- "Good security is always a mixture of prevention. detction, and response"[114] 
- We are willing to spend up to 90% of CPU on security, but costs of security can many times be hidden from user.
- Most of the time. the overhead increase from an efficient security system shouldnt be noticable on a human time scale; even when it's significant, that's just the cost of doing business.
- Security 1st, 2nd, 3rd, performance somwhere down the list. 
- Often realities of market > need for security. 

#### 1.10.2 Security Versus Features
> Complexity is the worst enemy of security.

- A computer program with 20 different bianary options (on/off) has more than a million configurations.
    - To get the program to work: test most common configurations.
    - To make program secure: must evaluate each of the million possible configurations.
- To make a large, simple system you have to create a clear and simple interafce between different parts of the system.
    - Complexity is a measure of how many things interact at any one point.

#### 1.10.3 Security Versus Evolving Systems
> One of the largest issues for security is that the system continues to evolve after the security mechanism is put in place. 

Designers need to consider a wider ranger of attackers and attack goals, while also anticipating and preparing for future uses of the system.

### 1.13 General Exercises
- [[Crytpo Eng Exercises]]

## Introduction to Cryptography
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
> 
 

 