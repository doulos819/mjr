### Survey of Current Resources
> Last week: Decided on general EPF project theme. Educational resources to support the learning pathway of perspective cryptography engineers. 
> This week: Survey of current applied cryptography/zk resources to aid in applying cryptography.

**Potential Problem:** There are many publicly available resources that are efficient at explaining how current (and historical) cryptographic systems work. There are also thorough courses available to support structured learning to teach core cryptography concepts. While these resources exist, it can take more time than is available (or reasonable) for engineers interested in learning to apply cryptography to go through them all (or to know where to find efficient resources).

**Potential Solution:** Create a resource that combines many already available resources, while also providing relevant examples that learners can interact with and build upon. 

**Intro (non-exhaustive):** Pre-Materials
1. Text: Free articles/blogs for background knowledge.
	- [EF1stP - Internet](https://explained-from-first-principles.com/internet/)
	- [EF1stP - Number Theory](https://explained-from-first-principles.com/number-theory/)
	- [indiscreetdiscretelog](https://thork.net/)
		- [Zero Knowledge: The Game](https://thork.net/posts/2020_zero_knowledge_game/)
		- [Walking Through Distributed Key Generation (DKG)](https://thork.net/posts/2022_4_21_dkg/)
	- [Vitalik Buterin's website](https://vitalik.ca/)
		- [Fast Fourier Transforms](https://vitalik.ca/general/2019/05/12/fft.html)
		- [Verkle trees](https://vitalik.ca/general/2021/06/18/verkle.html)
		- [Why sharding is great](https://vitalik.ca/general/2021/04/07/sharding.html)
		- [The Limits to Blockchain Scalability](https://vitalik.ca/general/2021/05/23/scaling.html)
		- [How do trusted setups work?](https://vitalik.ca/general/2022/03/14/trustedsetup.html)
		- [Endgame (Ethereum)](https://vitalik.ca/general/2021/12/06/endgame.html)
		- [Some ways to use ZK-SNARKs for privacy](https://vitalik.ca/general/2022/06/15/using_snarks.html)
		- [The different types of ZK-EVMs](https://vitalik.ca/general/2022/08/04/zkevm.html)
		- [QAP/ R1CS](https://medium.com/@VitalikButerin/quadratic-arithmetic-programs-from-zero-to-hero-f6d558cea649)
1. Video: Free Video Resources for basic background knowledge.
	- [Harvard OCW CS](https://youtu.be/8mAITcNt710)
	- [Rust w Boggy - Rust Book](https://www.youtube.com/playlist?list=PLai5B987bZ9CoVR-QEIN9foz4QCJ0H2Y8)
	- [Full Rust Course - Intermediate](https://www.youtube.com/watch?v=MsocPEZBd-M&t=541s&ab_channel=freeCodeCamp.org)
	- [Algebra](https://youtu.be/LwCRRUa8yTU)
	- [Mathematical Proofs]
		1. [Adam Panados](https://www.youtube.com/playlist?list=PLdciPPorsHuktAIU2ebPuFqxSvyBAkH57)
		2. [Albert R. Meyer - MIT OCW](https://www.youtube.com/playlist?list=PLUl4u3cNGP60UlabZBeeqOuoLuj_KNphQ)
	- [Number Theory](https://youtu.be/19SW3P_PRHQ)
	- [Pairings in Cryptography](https://youtu.be/8WDOpzxpnTE)
2. **Audio:** Free Audio Resources for basic background knowledge.
	- [ZKfm](https://zeroknowledge.fm/zero-knowledge-series/)

**Example:** Learn ZK 
1. Text: [A Graduate Course in Applied Cryptography](https://toc.cryptobook.us/book.pdf) - Dan Boneh and Victor Shoup
2. Video:  [ZK Whiteboard Sessions](https://www.youtube.com/playlist?list=PLj80z0cJm8QErn3akRcqvxUsyXWC81OGq)
3. Hybrid: [Coursera Crypto](https://www.coursera.org/learn/crypto)
   
____
### Monday

- [Crypto Engineering - 2010](https://github.com/doulos819/mjr/blob/main/research/Notes/books/Cryptography%20Engineering%20-%202010.md)
	- Finish ch 3 this week & start doing exercises.
- [ZK Whiteboard Sessions](https://www.youtube.com/playlist?list=PLj80z0cJm8QErn3akRcqvxUsyXWC81OGq)
	- Module 1 & 2.
- [ZKPorter](https://www.youtube.com/playlist?list=PLvvyxOd1rILcb4S3KkQn_V3Kx4iVI6Yw-)
	- [Class 3 - Tornado Cash Whitepaper](https://www.youtube.com/watch?v=jGmvJZ7m7WU&ab_channel=Porter)
- [A Graduate Course in Applied Cryptography](https://toc.cryptobook.us/book.pdf)
	- Started reading through text (pg 8)
	- Seems like an efficient pairing to CE - 2010. Valuable definitions, proofs, and mathematical exercises. 
	- More important than general notes would be finding definitions or proofs to help understand larger ideas & understanding the text/proofs.
-  [Coursera Crypto](https://www.coursera.org/learn/crypto)
	- [Corsera Crypto Week 1](https://www.coursera.org/learn/crypto/home/week/1)
	- Started the course to see what type of resources are available, have gone through a few videos.

### Tuesday

- Started the day with [Pairings in Cryptography](https://youtu.be/8WDOpzxpnTE) (dnf)
- Uncloak Team meeting - Study group starting next week hopefully. Also to start setting up beginnings of basic knowledge graph.
- [Corsera Crypto Week 1](https://www.coursera.org/learn/crypto/home/week/1)
	- Almost complete week 1 - looking forward to seeing how the python programming exercises are -> Rust. 

### Wednesday

- [Pairings in Cryptography](https://youtu.be/8WDOpzxpnTE)
	- Very insightful history of pairings (7 y/o vid tho)
- [Corsera Crypto Week 1](https://www.coursera.org/learn/crypto/home/week/1)
	- Finish W1 - Video material and questions are very efficient. 
	- Optional programming questions are in Python. 
		- Having these answered with Rust video walkthroughs would be very helpful.
	- Will continue with W2 Friday.
- [ZK Whiteboard Sessions](https://www.youtube.com/playlist?list=PLj80z0cJm8QErn3akRcqvxUsyXWC81OGq)
	- [Module Three: Building a SNARK, pt 2 by Dan Boneh](https://www.youtube.com/watch?v=vxyoPM2m7Yg&ab_channel=ZeroKnowledge)
		- I feel as though I finally understand the pieces that come together to build a SNARK (PLONK/IOP +PCS)
	- Want to start taking notes from these as well.

### Thursday

- [Corsera Crypto Week 2](https://www.coursera.org/learn/crypto/home/week/2)
	- Seems VERY similar to CR textbook so far. Will catch up by next week and then go through both resources.
	- Not sure the best way of taking notes, but will start next week with hand written noted (on tablet) from lectures to org into Obsidian notes later on.
		- Will try and link important Corsera notes to CE notes? (or vice versa)
- Meet with Mentor(Thor)
	- To start using Obsidian Publish soon
	- Starting on the knowledge graph
	- 2 weeks until study group start
	- Keep taking CE notes as well as Corsera
- [ZKPorter](https://www.youtube.com/playlist?list=PLvvyxOd1rILcb4S3KkQn_V3Kx4iVI6Yw-)
	- Class 4 - intro to programming SNARKS
	- ending Nov 10th -> CE Study group

### Friday

-  [Corsera Crypto Week 2](https://www.coursera.org/learn/crypto/home/week/2)
	- Finish week 2 vids
- [Crypto Engineering - 2010](https://github.com/doulos819/mjr/blob/main/research/Notes/books/Cryptography%20Engineering%20-%202010.md)
	- Stream completion of ch 3 notes
- [Full Rust Course - Intermediate](https://www.youtube.com/watch?v=MsocPEZBd-M&t=541s&ab_channel=freeCodeCamp.org)
	- set up "tutorials" repo
	- seems like a really fun one to do in browser in under 2 hr's
- [Euler in Rust](https://www.youtube.com/watch?v=12yU-onACsY&list=PLcFQxbPQBUEUNIsPQjHeULQnp3L3F_QwR&ab_channel=MarcusKazmierczak)
	- set up "euler" repo
	- need to practice math with Rust...
-  [QAP/ R1CS](https://medium.com/@VitalikButerin/quadratic-arithmetic-programs-from-zero-to-hero-f6d558cea649)
	- stream? at least read!
-  [ZK Whiteboard Sessions](https://www.youtube.com/playlist?list=PLj80z0cJm8QErn3akRcqvxUsyXWC81OGq)
	- Module 4

### Findings

- Corsera course seems like an efficient educational tool if paired with study groups/workshops.
	- Main issue is that current material is in Python + no guides for how to work through programming exercises.
	- Will work with Thor to get answers in Rust for programming exercises. 
- Many ideas build upon another, how to ensure efficient background knowledge for each "Model" that would be displayed. 
	- Knowledge graph will be obvious support here. 
	- If a current Model is too complex/large, how does a Model get broken down into more manageable/smaller pieces?
- Starting with having efficient notes to publish seems like an efficient way to support. 