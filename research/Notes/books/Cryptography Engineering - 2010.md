###### tags: `cryptography` `uncloak`
Author(s): Niels Ferguson, Bruce Schneier, and Tadayoshi Kohono

Paper(s): 
- paper to read?

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
- 
 