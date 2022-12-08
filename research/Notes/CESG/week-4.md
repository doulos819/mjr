# Session 4 

## Notes
> https://uncloak.org/courses/rust+cryptography+engineering/course-2022-12-09+Session+4+Notes

- This weeks reading will focus on Authenticated Encryption, the TLS Handshake, AEADs, GCM, and ChaCha20-Poly1305. (external to the book)
- [Authenticated encryption - Wikipedia](https://en.wikipedia.org/wiki/Authenticated_encryption) (**Authenticated Encryption** (**AE**) and **Authenticated Encryption with Associated Data** (**AEAD**) are forms of encryption which simultaneously assure the confidentiality and authenticity of data.)
	- [Galois/Counter Mode - Wikipedia](https://en.wikipedia.org/wiki/Galois/Counter_Mode#cite_note-1)
		- GCM is defined for block ciphers with a block size of 128 bits.
		- GCM can take full advantage of [parallel processing](https://en.wikipedia.org/wiki/Parallel_processing_(computing) "Parallel processing (computing)") and implementing GCM can make efficient use of an [instruction pipeline](https://en.wikipedia.org/wiki/Instruction_pipeline "Instruction pipeline") or a hardware pipeline.
	- [ChaCha20-Poly1305 - Wikipedia](https://en.wikipedia.org/wiki/ChaCha20-Poly1305#XChaCha20-Poly1305_%E2%80%93_extended_nonce_variant)
		- **ChaCha20-Poly1305** is an [authenticated encryption with additional data (AEAD)](https://en.wikipedia.org/wiki/Authenticated_encryption "Authenticated encryption") algorithm, that combines the [ChaCha20](https://en.wikipedia.org/wiki/Salsa20#ChaCha_variant "Salsa20") stream cipher with the [Poly1305](https://en.wikipedia.org/wiki/Poly1305 "Poly1305") message authentication code.
		-  It has fast software performance, and without hardware acceleration, is usually faster than AES-GCM.
	- [It takes two to ChaCha (Poly)](https://blog.cloudflare.com/it-takes-two-to-chacha-poly/)
		- [ChaCha-Poly](ChaCha-Poly.md)
- TLS - [TLS-SSL](TLS-SSL.md)
	- [Transport Layer Security - Wikipedia](https://en.wikipedia.org/wiki/Transport_Layer_Security#TLS_1.0)
		- The [protocol](https://en.wikipedia.org/wiki/Communication_protocol "Communication protocol") is widely used in applications such as [email](https://en.wikipedia.org/wiki/Email "Email"), [instant messaging](https://en.wikipedia.org/wiki/Instant_messaging "Instant messaging"), and [voice over IP](https://en.wikipedia.org/wiki/Voice_over_IP "Voice over IP"), but its use in securing [HTTPS](https://en.wikipedia.org/wiki/HTTPS "HTTPS") remains the most publicly visible.
	-  [What happens in a TLS handshake? | SSL handshake | Cloudflare](https://www.cloudflare.com/learning/ssl/what-happens-in-a-tls-handshake/)
	- [How does SSL work? | SSL certificates and TLS | Cloudflare](https://www.cloudflare.com/learning/ssl/how-does-ssl-work/)
	
	- Recommended libraries for this week:
		- [ChaCha20Poly1305 — Rust crypto library // Lib.rs](https://lib.rs/crates/chacha20poly1305)
		- [AES-GCM — Rust crypto library // Lib.rs](https://lib.rs/crates/aes-gcm)
		- [AEAD — Rust crypto library // Lib.rs](https://lib.rs/crates/aead)
	-   Optional Extra reading, 2007 paper introducing authenticated encryption as a primitive: [https://eprint.iacr.org/2000/025.pdf](https://eprint.iacr.org/2000/025.pdf)
		- [AE-Relations](AE-Relations.md)

## Exercises
> https://uncloak.org/courses/rust+cryptography+engineering/course-2022-12-09+Session+4+Notes#Exercises

1. Justify or disqualify each of the following schemes, with message $m$, tag $t$, and ciphertext $c$.
	- $t = MAC(m)$     $c = E(m)$, send $(c, t)$
	- $t = MAC(m)$     $c = E(m||t)$, send $c$
	- $c = E(m)$           $t = MAC(c)$, send $(c,t)$ 
2. You're the adversary, watching a TLS handshake. Pick three steps from [TLS Handshake - OSDev Wiki](https://wiki.osdev.org/TLS_Handshake#Handshake_Overview), and describe how the step prevents you from (pick one):
    -   reading message content (confidentiality)
    -   tampering with message content (integrity)
    -   impersonating either party (authenticity)