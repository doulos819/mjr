> https://uncloak.org/courses/rust+cryptography+engineering/course-2023-01-20+Session+8+Notes

This week's focus is Diffie Hellman key key?? exchange and RSA,

Extra Reading: https://github.com/trailofbits/publications/blob/master/papers/rsagtfo.pdf

New cryptography math textbook: https://drive.google.com/drive/u/0/folders/1ILBHUZrDZDku3HfK1yyp6AbBD_F3nRm5

Experience on elliptic curves: https://curves.xargs.org/

Intro to elliptic curves: https://research.nccgroup.com/2021/11/18/an-illustrated-guide-to-elliptic-curve-cryptography-validation/

Diffie Hellman (DH), relies on the difficulty of Discrete Log 
Difficult, in this case, means that no algorithm obtains $x$ in sub-polynomial time.


In additive groups, is typically , in multiplicative groups, is often . Note that this axiom guarantees that the set is non-empty. The identity is often written in multiplicative groups and in additive groups. does it need to be written twice?

For example, $3$ is a generator of $\mathbb Z_7$.

groups are commonly used in preference over prime fields, due to the existence of linear algebraic attacks on prime fields, known as index calculus methods (see ItMC 3.8, though the section is mathematically involved, and may not be easy to follow, out of context)

Chapter 11 presents an introduction to the DH protocol in the context of a multiplicative group. The chapter is somewhat dense, especially for the mathematically rusty, refer to 11.8 for the larger picture of the chapter.

safe primes

The explanation of using safe subgroups in section 11.6 is a bit evasive. The main take-away is that subgroups are used for efficiency reasons, as efficient attacks exist on prime-field cryptography, requiring to be thousands of bits,

The Chinese Remainder Theorem (CRT), is an algorithm to obtain $x$, given $a,b,p,q$ such that:

$$\begin{align}

x \equiv a \mod p\\

x \equiv b \mod q

\end{align}$$

- 8: $8=2^3$, and each of $2,4,6,8$ divide 8, thus $\varphi(8)=4$. This is also generalizable: for prime powers, $p^e$, $\varphi(p^e)= (p^{e-1})(p-1)$. For example, $27=3^3$, and we would expect 27 to have $27/3$ divisors. Thus $\varphi(27)= 3^3-3^2=3^2(3-1)=18$ .

