# Authenticated Encryption: Relations among notions and analysis of the generic composition paradigm - 2007
> https://eprint.iacr.org/2000/025.pdf

# Abstract
> An authenticated encryption scheme is a symmetric encryption scheme whose goal is to provide both privacy and integrity.

- Three composition methods are considered, namely Encrypt-and-MAC, MAC-then-encrypt, and Encrypt-then-MAC.
- For each of these, and for each notion of security, we indicate whether or not the resulting scheme meets the notion in question assuming the given symmetric encryption scheme is secure against chosen-plaintext attack and the given MAC is unforgeable under chosen-message attack.
- We provide proofs for the cases where the answer is “yes” and counter-examples for the cases where the answer is “no.”

# 1. Intro
> We use the term authenticated encryption scheme to refer to a shared-key based transform whose goal is to provide both privacy and authenticity of the encapsulated data.

- encryption process applied by the sender takes the key and a plaintext to return a ciphertext
- decryption process applied by the receiver takes the same key and a ciphertext to return either a plaintext or a special symbol indicating that it considers the ciphertext invalid or not authentic
- The first part of this paper formalizes several different possible notions of authenticity for symmetric encryption schemes, and integrates them into the existing mosaic of notions by relating them to the main known notions of privacy for symmetric encryption, via implications and separations in the style of [6] 
- The second part of this paper analyzes several generic composition methods with regard to meeting the previous notions

## 1.1 Relations among notions
> Privacy goals for symmetric encryption schemes include indistinguishability and non-malleability, each of which can be considered under either chosen-plaintext or (adaptive) chosen-ciphertext attack, leading to four notions of security we abbreviate IND-CPA, IND-CCA, NM-CPA, NM-CCA.

- We consider two notions of integrity (we use the terms authenticity and integrity interchangeably) for symmetric encryption schemes.
	- INT-PTXT (integrity of plaintexts) requires that it be computationally infeasible to produce a ciphertext decrypting to a message which the sender had never encrypted
	- INT-CTXT (integrity of ciphertexts) requires that it be computationally infeasible to produce a ciphertext not previously produced by the sender, regardless of whether or not the underlying plaintext is “new.”
	- The first of these notions is the more natural security requirement while the interest of the second, stronger notion is perhaps more in the implications we discuss below.
- These notions of authenticity are by themselves quite disjoint from the notions of privacy.
- To make for useful comparisons, we consider each notion of authenticity coupled with IND-CPA, the weakest notion of privacy; namely the notions on which we focus for comparison purposes are INT-PTXT ∧ IND-CPA and INT-CTXT ∧ IND-CPA. (Read “∧” as “and”.)
- Integrity of ciphertexts —even when coupled only with the weak privacy requirement IND-CPA— emerges as the most powerful notion.
	- Not only does it imply security against chosen-ciphertext attack, but it is strictly stronger than this notion.
- Non-malleability —whether under chosen-plaintext or chosen-ciphertext attack— does not imply any type of integrity. 
	- The intuitive reason is that non-malleability only prevents the generation of ciphertexts whose plaintexts are meaningfully related to those of some challenge ciphertexts, while integrity requires it to be hard to generate ciphertexts of new plaintexts even if these are unrelated to plaintexts underlying any existing ciphertexts.
- Finally, integrity of plaintexts does not imply integrity of ciphertexts.

## 1.2 Analysis of generic composition
> We focus in this paper on “generic composition:” simply combine a standard symmetric encryption scheme with an MA scheme in some way. There are a few possible ways to do it, and our goal is to analyze and compare their security.

- Generic composition. Assume we are given a symmetric encryption scheme $\mathcal{SE}$ whose encryption and decryption algorithms we denote by $\mathcal{E}$ and $\mathcal{D}$, respectively.
- Also assume we are given a message authentication scheme \mathcal{MA} whose tagging and tag-verifying algorithms we denote by \mathcal{T} and \mathcal{V}, respectively.
- We assume the encryption scheme meets the weakest notion of privacy, namely IND-CPA.
- We assume the MA scheme meets a notion of unforgeability under chosen message attack.