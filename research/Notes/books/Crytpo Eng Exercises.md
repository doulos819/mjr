# Crytpo Eng Exercises

> Ex for [[Cryptography Engineering - 2010]]

### Chapter 1
1. Create an attack tree for stealing a car.
2. Create attack tree for getting into gym without paying.
3. Create attack tree for getting food from restaurant without paying.
4. Create attack tree for learning someone's online banking account name and password.
5. Create attack tree for reading someone else's e-mail.
6. Create attack tree for preventing someone from reading their own e-mail.
7. Create attack tree for sending e-mail as someone else.
8. Find a new product or system that was announced or released recently. Construct a security review as described in Sec 1.12. Pick one asset, and construct attack tree for compromising that asset.
9. Provide concrete example in which attackers compromised a system by exploiting something other than the weakest link. Describe the system, what was the weakest link and why, how was the system compromised.
10. Describe a concrete example where improving the security of a system against one type of attack can increase the likelihood of other attacks


### Chapter 2
1. Consider Kerckhoffs' principle. Provide at least two arguments in favor and two against. Then state/argue your view of the validity of the principle. 
2. Suppose Alice and Bob are sending e-mails to each other over the Internet from laptops connected to free wifi.
	1. List all the parties who might be able to attack this system and what they might be able to accomplish.
	2. Describe how Alice and Bob might be able to defend against each.
3. Consider a group of 30 people who wish to establish pair-wise secure communications using symmetric-key cryptography. How many keys need to be exchanged in total. 
	- A - 435
4. Suppose Bob receives a messages signed using a digital signature scheme with Alice's secret signing key. Does it prove that Alice saw the message and chose to sign.
	- no
5. Suppose that PKIs do not exist. Suppose Alice obtains a pubic key $P$ that purportedly belongs to Bob. How can Alice develop confidence that $P$ really belongs to Bob? Consider the following:
	- Alice can talk with Bob over the phone.
	- Alice can talk with someone else she trusts over the phone (Charlie), and they have already verifies that $P$ belongs to Bob.
	- Alice can communicate with Charlie via e-mail, and Charlie has already verified that $P$ belongs to bob.
> explicitly state any addition assumtions
6. Suppose a chosen-ciphertext attacker cannot recover the seccret decryption key for an encryption scheme. Does this mean the encryption scheme is secure?
	- no
7. Consider a symmetric-key cryptosystem in which cryptographic keys are randomly selected from the set of all n-bit strings. Approximately what should n be in order to provide 128 bits of security against a birthday attack. 
	- $2^{64}$

### Chapter 3
1. 
