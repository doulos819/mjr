# It takes two ChaCha (Poly)
> https://blog.cloudflare.com/it-takes-two-to-chacha-poly/

## Intro
> 2016: Not long ago we introduced support for TLS cipher suites based on the [ChaCha20-Poly1305 AEAD](https://blog.cloudflare.com/do-the-chacha-better-mobile-performance-with-cryptography), for all our customers.  We introduced these cipher suites to give end users on mobile devices the best possible performance and security.

- This blog entry reviews the history of ChaCha20-Poly1305, its standardization process, as well as its importance for the future of the web; also its performance, compared to the other standard AEAD.

## What is an AEAD?
> ChaCha20-Poly1305 is an AEAD, Authenticated Encryption with Additional Data cipher. AEADs support two operations: "seal" and "open". Another common AEAD in use for TLS connections is AES-GCM.

- The "seal" operation receives the following as input:

	1.  The message to be encrypted - this is the plaintext.
	    
	2.  A secret key.
	    
	3.  A unique initialization value - aka the IV. 
		- It must be unique between invocations of the "seal" operation with the same key, otherwise the secrecy of the cipher is completely compromised.
	4.  Optionally some other, non-secret, additional data. 
		- This data will not be encrypted, but will be authenticated - this is the AD in AEAD.
- The "seal" operation uses the key and the IV to encrypt the plaintext into a ciphertext of equal length.
	- After the data is encrypted, "seal" uses the key (and optionally the IV) to generate a secondary key.
	- The secondary key is used to generate a keyed hash of the AD, the ciphertext and the individual lengths of each.
		- The hash used in ChaCha20-Poly1305, is Poly1305 and in AES-GCM the hash is GHASH.
	- The final step is to take the hash value and encrypt it too, generating the final MAC (Message Authentication Code) and appending it to the ciphertext.
- The "open" operation is the reverse of "seal". 
	- It takes the same key and IV and generates the MAC of the ciphertext and the AD, similarly to the way "seal" did. 
	- It then reads the MAC appended after the ciphertext, and compares the two.

### What makes AEADs special?
> AEADs are special in the sense that they combine two algorithms - cipher and MAC, into a single primitive, with provable security guarantees.

- Before AEADs, it was acceptable to take some cipher and some MAC, which were considered secure independently, and combine them into an insecure combination.
- For example some combinations were broken by reusing the same keys for encryption and MAC (AES-CBC with CBC-MAC), while others by performing MAC over plaintext instead of the ciphertext [AES-CBC with HMAC in TLS](https://blog.cloudflare.com/padding-oracles-and-the-decline-of-cbc-mode-ciphersuites/).

### The new kid in the block?
> ChaCha20 & Poly1305 are both the brain children of Daniel J. Bernstein (DJB). ChaCha20 itself was published in 2008. Poly1305 was published in 2004. It slightly modifies the Salsa round, and the number 20 indicates that it repeats for 20 rounds in total.

- ChaCha20 is a 256-bit cipher based upon an earlier cipher developed by DJB called Salsa, that dates back to 2005, and was submitted to the eSTREAM competition.
	- It slightly modifies the Salsa round, and the number 20 indicates that it repeats for 20 rounds in total.
- Similar to AES-CTR, ChaCha20 is a stream cipher.
	- It generates a pseudo-random stream of bits from an incremented counter, the stream is then "XORed" with plaintext to encrypt it (or "XORed" with ciphertext to decrypt).
	- Because you do not need to know the plaintext in advance to generate the stream, this approach allows both to be very efficient and parallelizable. 
- Poly1305 is a MAC, and can be used with any encrypted or unencrypted message, to generate a keyed authentication token.
	- Originally Poly1305 used AES as the underlying cipher (Poly1305-AES); now it uses ChaCha20. Again, similarly to GHASH, it is a polynomial evaluation hash.
	-  Unlike GHASH, its key changes for each new message, because it depends on the IV.
- (DJB is also responsible for Curve25519 key exchange).

### From zero to hero
> The body that governs internet standards is the IETF - Internet Engineering Task Force. All the standards we use on the internet, including TLS, come from that organization.

- The standardization process is open to all, and the correspondence that relates to it is kept public in a special [archive](https://mailarchive.ietf.org/arch/).
	- The first mention for ChaCha20-Poly1305 I found in the archive dates to [30 July 2013](https://www.ietf.org/mail-archive/web/tls/current/msg09707.html). It is still referred to as Salsa back then.
	-  The latest draft of the ChaCha20-Poly1305 for TLS including all the previous revisions can be found [here](https://datatracker.ietf.org/doc/draft-ietf-tls-chacha20-poly1305/).
		- Another standard that defines the general usage of ChaCha20-Poly1305 is [RFC7539](https://datatracker.ietf.org/doc/rfc7539/). First published in January 2014, it was standardized in May 2015.

### Differences
> There are two key differences between the draft version we initially implemented and the current version of the cipher suites that make the two incompatible.

- The first difference relates to how the cipher suite is used in TLS. The current version incorporated the TLS records sequence number into the IV generation, making it more resistant to dangerous IV reuse.
- The second difference relates to how Poly1305 applies to the TLS record. Records are the equivalent of a TCP packet for TLS.
- The older cipher suites can be identified by IDs {cc}{13}, {cc}{14} and {cc}{15}, while the newer cipher suites have IDs {cc}{a8} through {cc}{ae}.

### Future of ChaCha20-Poly1305
> Today we already see that almost 20% of all the request to sites using CloudFlare use [ChaCha20-Poly1305](https://blog.cloudflare.com/padding-oracles-and-the-decline-of-cbc-mode-ciphersuites/). (current numbers are?)

- the IETF is currently finalizing another very important standard, [TLS 1.3](https://blog.cloudflare.com/going-to-ietf-95-join-the-tls-1-3-hackathon/). Right now it looks like TLS 1.3 will allow AEADs only
	- Leaving AES-GCM and ChaCha20-Poly1305 as the only two options (for now).

### Can you handle it?
> Most of our servers are based on Intel CPUs with 256-bit SIMD extensions called AVX2. We utilize those to get the maximal performance.

- The main competition for ChaCha20-Poly1305 are the AES-GCM based cipher suites.
	- The most widely used AES-GCM, uses AES with 128 bit key, however in terms of security AES-256 is more comparable to ChaCha20. WHY?
- In practice it means many connections will never reach the maximal size of a TLS record (16KB), but instead will use significantly smaller records (below 1400 bytes).
- AES-128-GCM and AES-256-GCM both still beat ChaCha20-Poly1305 in pure performance for records larger than 320 bytes
	-  Getting below 2 cycles/byte is a major performance achievement.****

### Performance outlook
> The current performance is outstanding.

- In the future, processors with wider SIMD instructions are expected to bridge the performance gap. The AVX512 will provide instructions twice as wide, and potentially will improve the performance two fold as well, bringing it below 1 cycle/byte.

### Conclusion
> CloudFlare is constantly pushing the envelope in terms of TLS performance and availability of the most secure cipher suites and modes. We are actively involved in the development and specification of TLS 1.3 and are committed to open source by releasing our performance patches.