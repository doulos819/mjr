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
- Proposition 1.4. Let a, b, c ∈ Z be integers.  
	- (a) If a | b and b | c, then a | c.  
	- (b) If a | b and b | a, then a = ±b.  
	- (c) If a | b and a | c, then a | (b + c) and a | (b − c).
- Definition. A common divisor of two integers a and b is a positive integer d that divides both of them.
- The greatest common divisor of a and b is, as  its name suggests, the largest positive integer d such that d | a and d | b
- t ≤ 2k, so the Euclidean algorithm terminates in at most 2k  
iterations.
	- Choose the smallest such k, so 2k ≥ b > 2k−1. Then: # of iterations $≤ 2k = 2(k - 1) + 2 < 2 log2(b) + 2$ 
	- the Euclidean algorithm applied to a and b with a ≥ b requires no more than 2 log2(b) + 1 iterations to compute gcd(a, b).
		- actual range is better
- Definition. Let a and b be integers. We say that a and b are relatively prime if gcd(a, b) = 1.

## 1.3 Modular Arithmetic
- Definition. Let m ≥ 1 be an integer. We say that the integers a and b are congruent modulo m if their difference a − b is divisible by m. We write a ≡ b (mod m) to indicate that a and b are congruent modulo m. The number m is called the modulus.
- Proposition 1.13. Let m ≥ 1 be an integer. 
	- (a) If a1 ≡ a2 (mod m) and b1 ≡ b2 (mod m), then a1 ± b1 ≡ a2 ± b2 (mod m) and a1 · b1 ≡ a2 · b2 (mod m). 
	- (b) Let a be an integer. Then a · b ≡ 1 (mod m) for some integer b if and only if gcd(a, m)=1
- Suppose first that gcd(a, m) = 1. Then Theorem 1.11 tells us that we can find integers u and v satisfying au + mv = 1. This means that au − 1=−mv is divisible by m, so by definition, au ≡ 1 (mod m). In other words, we can take b = u.
- Show $a^{-1}$ exists by using EEA
- $\mathbb{Z}/m\mathbb{Z}$ the ring of integers modulo m.
- More succinctly, if we let 
	- p = Plaintext Letter, c = Ciphertext Letter, k = Secret Key
	- then c ≡ p + k (mod 26) and p ≡ c − k (mod 26)

### 1.3.2 The fast powering algorithm
- 