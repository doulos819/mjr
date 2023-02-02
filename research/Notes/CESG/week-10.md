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
- Alice and Bob are required to compute large powers of a number g modulo another number N , where N may have hundreds of digits. The naive way to compute gA is by repeated multiplication by g.
- if A is large, this algorithm is completely impractical. For example, if A ≈ 21000, then the naive algorithm would take longer than the estimated age of the universe!
- The idea is to use the binary expansion of the exponent A to convert the calculation of $g^A$ into a succession of squarings and multiplications.
- Example 1.19. 
	- Suppose that we want to compute 3218 (mod 1000). The first step is to write 218 as a sum of powers of 2, $218 = 2 + 2^3 +2^4 +2^6 +2^7$.
	- Then $3^{218}$ becomes $3^{218} =3^2+{}^{2^3}+{}^{2^4}+{}^{2^6}+{}^{2^7} =3^2 · 3^{2^3} · 3^{2^4} · 3^{2^6} · 3^{2^7}$ . (each number in the sequence is the square of the preceding one)
	- we may reduce modulo 1000 after each multiplication.

## 1.4 Prime numbers, unique factorization, and finite fields
- Definition. An integer p is called a prime if p ≥ 2 and if the only positive integers dividing p are 1 and p.
- Proposition 1.20. Let p be a prime number, and suppose that p divides the product ab of two integers a and b.Then p divides at least one of a and b.
- Theorem 1.21 (The Fundamental Theorem of Arithmetic). Let a ≥ 2 be an integer. Then a can be factored as a product of prime numbers
	- $a = p_1^{e_1} · p^{e_2}_2 · p^{e_3}_3 ···p^{e_r}_r.$ 
	- Further, other than rearranging the order of the primes, this factorization into prime powers is unique.
	- (For an example of a situation in which unique factorization fails to be true, see the E-zone described in [126, Chatper 7].)
	- each prime p appears to a particular power. We denote this power by $ord_p(a)$ and call it the order (or exponent) of p in a.  appears to a particular power. We denote this power by ord(For convenience, we set $ord_p(1) = 0$ for all primes.)
		- For example, the factorization of 1728 is 1728 = 26 · 33,so ord2(1728) = 6, ord3(1728) = 3, and ordp(1728) = 0 for all primes p ≥ 5. ***
	- We now observe that if p is a prime, then every nonzero number modulo p has a multiplicative inverse modulo p.
	- Proposition 1.22. Let p be a prime. Then every nonzero element a in $\mathbb{Z}/p\mathbb{Z}$ has a multiplicative inverse, that is, there is a number b satisfying
		- ab ≡ 1 (mod p).
		- We denote this value of b by $a^{−1}$ mod p , or if p has already been specified, then simply by $a^{−1}$.
		- Proof. This proposition is a special case of Proposition 1.13(b) using the prime modulus p, since if a ∈ Z/pZ is not zero, then gcd(a, p)=1.
		- Remark 1.23. The extended Euclidean algorithm (Theorem 1.11) gives us an efficient computational method for computing a−1 mod p. We simply solve the equation au + pv = 1 in integers u and v, and then u = a−1 mod p.
		- Definition. If p is prime, then the set $\mathbb{Z}/p\mathbb{Z}$ of integers modulo p with its addition, subtraction, multiplication, and division rules is an example of a field.
	- For $a, b ∈ \mathbb{F}p$, the equality of a and b is denoted by a = b, while for $a, b ∈ \mathbb{Z}/p\mathbb{Z}$, the equality of a and b is denoted by equivalence modulo p, i.e., a ≡ b (mod p).

## 1.5 Powers and primitive roots in finite fields
- Theorem 1.25 (Fermat’s Little Theorem). Let p be a prime number and let a be any integer. Then
	- $a^{p−1} ≡  1 (mod p)$ if $p \nshortmid a$,
	- $0 (mod p)$ if $p | a$.
- Example 1.26. 
	- The number p = 15485863 is prime, so Fermat’s little theorem (Theorem 1.25) tells us that $2^{15485862} ≡ 1$ (mod 15485863).
	- Thus without doing any computing, we know that the number $2^{15485862} − 1$, a number having more than two million digits, is a multiple of 15485863.
- Remark 1.27. Fermat’s little theorem (Theorem 1.25) and the fast powering algorithm (Section 1.3.2) provide us with a reasonably efficient method of computing inverses modulo p, namely
	- $a^{−1} ≡ a^{p−2}$ (mod p).
	- true because if we multiply $a^{p−2}$ by a, then Fermat’s theorem tells us that the product is equal to 1 modulo p.
- Proposition 1.30. 
	- Let p be a prime and let a be an integer not divisible by p. 
		- Suppose that $a^n ≡ 1$ (mod p). Then the order of a modulo p divides n. In particular, the order of a divides p − 1.
- Theorem 1.31 (Primitive Root Theorem). 
	- Let p be a prime number. Then there exists an element $g ∈ F^∗_p$ whose powers give every element of $F^∗_p$, i.e., $F^∗_p = \{1,g,g^2,g^3,...,g^{p−2}\}$. Elements with this property are called primitive roots of $F_p$ or generators of $F^∗_p$. They are the elements of $F^∗_p$ having order p − 1.
	- 