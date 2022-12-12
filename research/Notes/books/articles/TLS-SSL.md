# TLS & SSL
>1. https://www.cloudflare.com/learning/ssl/how-does-ssl-work/
>2. https://www.cloudflare.com/learning/ssl/what-happens-in-a-tls-handshake/ 

# Intro
The main use case for SSL/TLS is securing communications between a client and a server, but it can also secure email, VoIP, and other communications over unsecured networks.

# Notes

## SSL
> [SSL](https://www.cloudflare.com/learning/ssl/what-is-ssl/) stands for Secure Sockets Layer, and it refers to a protocol for encrypting, securing, and authenticating communications that take place on the Internet. Although SSL was replaced by an updated protocol called [TLS (Transport Layer Security)](https://www.cloudflare.com/learning/ssl/transport-layer-security-tls/) some time ago, "SSL" is still a commonly used term for this technology.

## TLS
> Secure communication begins with a [TLS handshake](https://www.cloudflare.com/learning/ssl/what-happens-in-a-tls-handshake/), in which the two communicating parties open a secure connection and exchange the public key

- The two parties generate session keys
- Different session keys are used to encrypt communications in each new session
- TLS ensures that data has not been altered, and that the party is actually who they claim to be. 
- Encrypted data has to be decrypted by the recipient using a key.

## What is a cipher suite?
> A cipher suite is a set of algorithms for use in establishing a secure communications connection. There are a number of cipher suites in wide use, and an essential part of the TLS handshake is agreeing upon which cipher suite will be used for that handshake.

## What happens in a TLS handshake? | SSL handshake
> In a TLS/SSL handshake, clients and servers exchange SSL certificates, cipher suite requirements, and randomly generated data for creating session keys.

- [TLS](https://www.cloudflare.com/learning/ssl/transport-layer-security-tls/) is an encryption and authentication protocol designed to secure Internet communications. 
- A TLS handshake is the process that kicks off a communication session that uses TLS. 
- During a TLS handshake, the two communicating sides exchange messages to acknowledge each other, verify each other, establish the cryptographic algorithms they will use, and agree on session keys. 
- TLS handshakes are a foundational part of [how HTTPS works](https://www.cloudflare.com/learning/ssl/what-is-https/).

## TLS vs. SSL handshakes
> [SSL](#SSL)

### The TLS handshake 
> A TLS handshake uses asymmetric encryption, meaning that two different keys are used on the two ends of the conversation. This is possible because of a technique called public key cryptography.

- Data encrypted with the public key can only be decrypted with the private key.
- The client and server use the public and private keys to exchange randomly generated data used to create new keys for encryption, called the session keys.

### What happens during a TLS handshake?
> Specify which version of TLS, Decide on which cipher suites, Authenticate the identity of the server, Generate session keys in order to use symmetric encryption

#### What are the steps of a TLS handshake?
> A TLS handshake involves multiple steps, as the client and server exchange the information necessary for completing the handshake and making further conversation possible.

- The exact steps within a TLS handshake will vary depending upon the kind of key exchange algorithm used and the cipher suites supported by both sides.
- The RSA key exchange algorithm, while now considered not secure, was used in versions of TLS before 1.3. It goes roughly as follows:
	1. **The 'client hello' message:** The message will include which TLS version the client supports, the cipher suites supported, and a string of random bytes known as the "client random."
	2. **The 'server hello' message:** containing the server's [SSL certificate](https://www.cloudflare.com/learning/ssl/what-is-an-ssl-certificate/), the server's chosen cipher suite, and the "server random".
	3. **Authentication:** The client verifies the server's SSL certificate with the certificate authority that issued it.
	4. **The premaster secret:**  The client sends one more random string of bytes, the "premaster secret." The premaster secret is encrypted with the public key and can only be decrypted with the private key by the server.
	5. **Private key used:** The server decrypts the premaster secret.
	6. **Session keys created:** Both client and server generate session keys from the client random, the server random, and the premaster secret.
	7.  **Client is ready:** The client sends a "finished" message encrypted with a session key.
	8.  **Server is ready:** The server sends a "finished" message encrypted with a session key.
	9.  **Secure symmetric encryption achieved:** The handshake is completed, and communication continues using the session keys.

- Not all TLS handshakes will use the private key in the process of generating session keys. For instance, an ephemeral Diffie-Hellman handshake proceeds as follows:
	1. **Client hello:** The client sends a client hello message with the protocol version, the client random, and a list of cipher suites.
	2.  **Server hello:** The server replies with its SSL certificate, its selected cipher suite, and the server random. In contrast to the RSA handshake described above, in this message the server also includes the following (step 3):
	3.  **Server's digital signature:** The server computes a digital signature of all the messages up to this point.
	4.  **Digital signature confirmed:** The client verifies the server's digital signature, confirming that the server is who it says it is.
	5.  **Client DH parameter:** The client sends its DH parameter to the server.
	6.  **Client and server calculate the premaster secret:** Instead of the client generating the premaster secret and sending it to the server, as in an RSA handshake, the client and server use the DH parameters they exchanged to calculate a matching premaster secret separately.
	7.  **Session keys created:** Now, the client and server calculate session keys from the premaster secret, client random, and server random, just like in an RSA handshake.
	8.  **Client is ready:** Same as an RSA handshake.
	9.  **Server is ready**
	10.  **Secure symmetric encryption achieved**

- The Diffie-Hellman algorithm uses exponential calculations to arrive at the same premaster secret.
- https://www.cloudflare.com/learning/ssl/keyless-ssl/:  contrast between ephemeral Diffie-Hellman handshakes and other kinds of handshakes, and how they achieve forward secrecy

### What is different about a handshake in TLS 1.3?
> TLS 1.3 does not support RSA, nor other cipher suites and parameters that are vulnerable to attack. It also shortens the TLS handshake.

-  **Client hello:** Because support for insecure cipher suites has been removed the client is assuming that it knows the server’s preferred key exchange method. The client hello also includes the parameters that will be used for calculating the premaster secret. Essentially, . This cuts down the overall length of the handshake.
- **Server generates master secret:** At this point, the server has received the client random and the client's parameters and cipher suites. Therefore, the server can create the master secret.
- **Server hello and "Finished":** The server hello includes the server’s certificate, digital signature, server random, and chosen cipher suite.
- **Final steps and client "Finished":** Client verifies signature and certificate, generates master secret, and sends "Finished" message.
- **Secure symmetric encryption achieved**

#### 0-RTT mode for session resumption
>  If the client and the server have connected to each other before (as in, if the user has visited the website before), they can each derive another shared secret from the first session, called the "resumption main secret."

- The server also sends the client a session ticket during this first session. 
- The client can use this shared secret to send encrypted data to the server on its first message of the next session, along with that session ticket. 

#### Symmetric encryption with session keys
> Unlike asymmetric encryption, in symmetric encryption the two parties in a conversation use the same key.

- Once session keys are in use, the public and private keys are not used anymore.
- ![Symmetric Encryption](https://cf-assets.www.cloudflare.com/slt3lc6tev37/1PYEAgdkoII5tQ5yzweHEX/a025977d2cb6a74df020ceb6273ae6d5/symmetric-encryption.svg)
- A new, random set of session keys will be created for the next session.

## Authenticating the origin server
> TLS communications from the server include a MAC, which is a digital signature confirming that the communication originated from the actual website.

## What is an SSL certificate?
> An SSL certificate is a file installed on a website's [origin server](https://www.cloudflare.com/learning/cdn/glossary/origin-server/). 

- Without an SSL certificate, a website's traffic can't be encrypted with TLS.
- However, browsers do not consider self-signed certificates to be as trustworthy as SSL certificates issued by a certificate authority.

## How does a website get an SSL certificate?
> Website owners need to obtain an SSL certificate from a certificate authority, and then install it on their web server (often a web host can handle this process).

## Is it possible to get a free SSL certificate?
> Many certificate authorities charge for SSL certificates. To help make the Internet more secure, Cloudflare offers [free SSL certificates](https://www.cloudflare.com/ssl/).

-  For more information about SSL options with Cloudflare, see our [Developer documentation](https://developers.cloudflare.com/ssl/).

## What is the difference between HTTP and HTTPS?
> The S in "HTTPS" stands for "secure." HTTPS is just HTTP with SSL/TLS.

- A website with an HTTPS address has a legitimate SSL certificate issued by a certificate authority, and traffic to and from that website is authenticated and encrypted with the SSL/TLS protocol.
- [Test a website for SSL/HTTPS issues](https://www.cloudflare.com/diagnostic-center/).
- o encourage the Internet as a whole to move to the more secure HTTPS, many web browsers have started to mark HTTP websites as "not secure" or "unsafe".

