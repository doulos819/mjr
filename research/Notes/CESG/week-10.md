# Starting 4 weeks covering sections from An Introduction to Mathematical Cryptography 
> uncloak notes: https://uncloak.org/courses/rust+cryptography+engineering/course-2023-02-03+Session+10+Notes

page 19, modular Fast Powering Algorithm, (1.11a), (1.18), after gcd =1 is known find g.

1.2-1.5, to develop intuition for properties and algorithms on the modular integers.

Euler's totient function and the Fast Powering Algorithm.

understand Proposition 1.22 (You may need to revisit the Extended Euclidean Algorithm if it is unclear)

Fermat's Little Theorem, and the Primitive Root Theorem, which guarantees the existence of a generator


# An Introduction to Mathematical Cryptography 
> textbook: https://drive.google.com/drive/u/0/folders/1ILBHUZrDZDku3HfK1yyp6AbBD_F3nRm5

## Introduction
> A Principal Goal of (Public Key) Cryptography  
is to allow two people to exchange confidential information,  
even if they have never met and can communicate only via  
a channel that is being monitored by an adversary.

## 1.1  Simple substitution ciphers
*definition:* Cryptography, the methodology of concealing the content of messages, comes from the Greek root words kryptos, meaning hidden, and graphikos, meaning writing.

*definition:* Caesar/Shift Cipher: each letter in the alphabet is shifted up or down.

*definition:* simple substitution cipher: a cipher in which  
each letter is replaced by another letter (or some other type of symbol). The Caesar cipher is an example of a simple substitution cipher, but there are many simple substitution ciphers other than the Caesar cipher

*definition:* Note that in order for decryption to work, the encryption function must have the property that no two plaintext letters go to the same ciphertext letter. A function with this property is said to be one-to-one or injective.

### 1.1.1 Cryptanalysis of simple substitution ciphers
- The process of decrypting a message without knowing the underlying key is called crypt-  
analysis.
- Your opponent always uses her best strategy to defeat you,  
not the strategy that you want her to use. Thus the security of an encryption system depends on the best known method to break it. As new and improved methods are developed, the level of security can only get worse, never better.

## 1.2 Divisibility and greatest common divisors
> Number Theory is the study of ints {-2, -1, 0, 1, 2} denoted by the symbol $\mathbb{Z}$.
- The set of integers with their addition and multiplication rules are an example of a ring.
	- The property of staying inside of our original set after applying operations to a pair of elements is characteristic of a ring.
- Definition. A common divisor of two integers a and b is a positive integer d that divides both of them.
- The greatest common divisor of a and b is, as  its name suggests, the largest positive integer d such that d | a and d | b