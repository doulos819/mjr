# Lessons From The History Of Attacks On Secure Hash Functions
> https://web.archive.org/web/20220708064142/https://electriccoin.co/blog/lessons-from-the-history-of-attacks-on-secure-hash-functions/

## Summary
> _This document is a work-in-progress. Please contact the author if you see errors or omissions._

- The main result is that there is a big gap between the history of collision attacks and pre-image attacks. Almost _all_ older secure hash functions have fallen to collision attacks. Almost _none_ have ever fallen to pre-image attacks.
- Secondarily, no _new_ secure hash functions (designed after approximately the year 2000) have so far succumbed to collision attacks, either.

## Preliminaries
> The input to a secure hash function is called the _pre-image_ and the output is called the _image_.

- A hash function _collision_ is two different inputs (pre-images) which result in the same output. A hash function is _collision-resistant_ if an adversary can’t find any collision.
- A hash function is _pre-image resistant_ if, given an output (image), an adversary can’t find any input (pre-image) which results in that output.
	- A hash function is _second-pre-image resistant_ if, given _one_ pre-image, an adversary can’t find any _other_ pre-image which results in the same image.

## When collision attacks don’t matter
> There are cases where collision-resistance doesn’t matter at all and what you care about is second-pre-image resistance.

- For this purpose the relevant question is not whether hash function designs have historically been revealed to be vulnerable to collisions but instead whether they’ve been revealed to be vulnerable to (second-)pre-images.

### hash-based digital signatures
> An example of this is the construction of hash-based digital signatures. Hash-based digital signatures are secure (resistant to forgery) as long as the hash function they are built on has second-pre-image resistance, e.g. [SPHINCS](https://web.archive.org/web/20220708064142/https://sphincs.cr.yp.to/sphincs-20150202.pdf).

- Such a hash-based digital signature would fail if its underlying hash function failed at second-pre-image resistance, but this is the _only_ way that it could be broken—any attack which was able to forge digital signatures against such a scheme would _have_ to violate the second-pre-image resistance of the underlying hash function.
	- If an attacker has a sufficiently large quantum computer, they could forge digital signatures that rely on factorization or discrete log, such as RSA, DSA, ECDSA, or Ed25519. There is no reason to think that such a quantum computer would enable them to break secure hash functions, however.
	- A mathematical breakthrough that allows them to exploit the asymmetric crypto technique (such as factoring, discrete log, code-based crypto, etc.), then they would be able to exploit asymmetric-crypto-based digital signatures, but not hash-based digital signatures.
- Any attacker who can break hash-based signatures can also break asymmetric-crypto-based signatures, because the latter rely on hash functions in addition to relying on their asymmetric crypto primitives.

## When collision attacks _do_ matter
> Be careful about this! The ability to generate collisions can be surprisingly harmful to many systems. The famous “Internet Root Cert” attack [[18]](https://web.archive.org/web/20220705235340/https://www.win.tue.nl/hashclash/rogue-ca/) is an example of engineers working at VeriSign incorrectly thinking that their system was not threatened by collisions (in the absence of second-pre-images).

- git, which uses SHA-1, is like VeriSign’s MD5 certificates in this way—it is _believed_ by its developers [[50]](https://web.archive.org/web/20220626122908/https://www.mail-archive.com/cryptography@metzdowd.com/msg10800.html) that a mere collision attack (not second-pre-image) against SHA-1 wouldn’t make git users vulnerable to malicious action, but no-one has written a security proof showing that git is safe against this attack

## Results
> _The bottom line is that no widely-studied hash function has ever succumbed to a (second-)pre-image attack except for one._ That single exception is the second-oldest secure hash function ever designed, _Snefru_, which was designed in 1989 and 1990, and which turned out to be vulnerable to differential cryptanalysis.

- The history of (second-)pre-image attacks is therefore quite different from the history of collision attacks. Most hash functions have been proven vulnerable to collision attacks more efficient than brute force, and even to collision attacks that could be implemented in practice.

## History of attacks on hash functions
> I omit attacks on reduced-round or otherwise weakened variants of hash functions (there are a lot of those). I omit attacks that have unrealistic requirements, like attacks that require $2^{128}$ precomputation or require the messages to be $2^{56}$ blocks long (there are a lot of those, too).

- Any hash function From Whirlpool (2000) onwards should be safe to use. No known attacks. 
- BLAKE2 is the most recent addition (https://web.archive.org/web/20220708064142/https://blake2.net/)
- If you are aware of any other papers which fit these criteria, or if you spot an error in this document, please write to me: [zooko@z.cash](mailto:zooko@z.cash)
- _legend:_
	-   _bit_: the number of bits of output
	-   _cpb_: cycles per byte [*]
	-   _comp_: approximate computation required for the attack
	-   _mem_: approximate memory required for the attack

## Discussion
> The main result of this investigation is that there is a big gap between the historical successes of collision attacks and the almost non-existence successes of pre-image attacks. This is evidence that a cryptosystem invulnerable to collision attacks is much safer than one that is vulnerable to collision attacks (regardless of whether it is vulnerable to pre-image attacks).
- Another interesting pattern that I perceive in these results is that _maybe_ sometime between 1996 (Tiger) and 2000 (Whirlpool), humanity learned how to make collision-resistant hash functions, and none of the prominent secure hash functions designed since that era have succumbed to collision attacks.
	- Maybe modern hash functions like SHA-256, SHA-3, and BLAKE2 will never be broken.
	- Or maybe this is just a 17-year-long hiatus, and in the future we’ll discover how to perform collision attacks against the “modern” secure hash functions. Looking in the rearview mirror can’t answer that for us.