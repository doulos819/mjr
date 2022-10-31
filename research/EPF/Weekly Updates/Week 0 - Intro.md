### Project Setup & Learning

#### Intro
- My name is [Chloe](https//:marmaj.org/chloe), and I am very excited to be taking part in the Ethereum Protocol Fellowship over the next 4 months ([specifically cohort 3](https://github.com/eth-protocol-fellows/cohort-three/blob/master/program-guide/program-details.md)).
- My background is in Research more than engineering, but I hope to use the next four months to try and bridge the gap.
- My main areas of interest are privacy/zk (specifically validator privacy)
- Hours of availability 8am-5pm EST

#### Goals
- Better understanding of implementing practical cryptography in systems [Cryptography Engineering - 2010](https://drive.google.com/drive/folders/1506sz7G5o6ATeGObP1AEwMV4msaLK3HD?usp=sharing)
	- Start up Crypto Eng Study Group
- Be able to support a team to implement protocol specs in Rust [[Zero2Prod (in Rust) - 2022]] (Wakuv2?)
- Some teams I am interested in:
	- [Status](https://status.im/) - resource constant devices 
	- [Vacp2p](https://vac.dev/) - private messaging
	- [Portal Network](https://github.com/ethereum/portal-network-specs) - light clients

#### Plans
- Work with [Thor](https://twitter.com/cryptograthor) as my mentor as I work to implement cryptography widgets in Rust
	- Uncloak - Build Rust backend to host educational content. 
	- Cryptography engineering [study group](https://discord.gg/gVAaf8kH) soon™️  
- Curate [[Reading List]] (https://github.com/doulos819/mjr/blob/main/research/EPF/Reading%20List.md) and have efficient notes on many of the resources.
____
### Pre-Fellowship
- Was spent choosing a mentor (Thor), and starting to go through the reading materials. I also have been gathering resources to support with the "Project" that I hope to work on over the next 4 months (and hopefully beyond). 
- Also trying to focus on some personal goals over the next 4 months to try and not burn out... again...:
	- Attempt to stick to ~20h/w (also m-f)
	- Learn in a community as much as possible
		- Open office hours & study groups
- Lots of brushing up on Rust/Number Theory

## Week 0 (2022.10.24)

### Monday 
- Get started using [Obsidian](https://obsidian.md/)
	- Relatively efficient so far
	- Useful for having data stored locally
	- Brushing up on [LaTeX](https://katex.org/docs/supported.html)
- Working through a bit of [Cryptography Engineering - 2010](https://drive.google.com/drive/folders/1506sz7G5o6ATeGObP1AEwMV4msaLK3HD?usp=sharing)
	- Reading progress: ch3
	- Streaming/[note](https://github.com/doulos819/mjr/blob/main/research/Notes/books/Cryptography%20Engineering%20-%202010.md) progress: ch2
- Set up [Reading list](https://github.com/doulos819/mjr/blob/main/research/EPF/Reading%20List.md)
  
### Tuesday
  - Held [office hours](https://marmaj.org/events/marma-j-research-dao-office-hours/)
	  - Discussed with HS student who was interested in becoming Ethereum Researcher and she will join CE study group
- Started Setting up [Zero2Prod Notes](https://github.com/doulos819/mjr/blob/main/research/Notes/books/Zero2Prod%20(in%20Rust)%20-%202022.md)

### Wednesday
- Crypto Eng Stream/Notes
	- Setup page for [exercises](https://github.com/doulos819/mjr/blob/main/research/Notes/books/Crytpo%20Eng%20Exercises.md); this is a very important part of the learning process. 
	- Continue to work through ch 2
	- Met with Thor; study group should start in 2 weeks. 

### Thursday
- Started attending (and caught up) zkp open class:
	- [ZK Class: Introduction to ZKP](https://www.youtube.com/watch?v=-2qHqfqPeR8&ab_channel=Porter)
	- [Zero Knowledge Proofs in Practice: Ethereum, Scroll, zkSync, StarkNET, Aztec, Zcash, Mina,Manta,Aleo](https://www.youtube.com/watch?v=sYINY-kruw0&t=149s&ab_channel=Porter)
- Starting to attend EF open calls again. Plan is to attend (and catch up) on recent calls to get situated on ways to potentially support EIP-4844.
	- [EIP-4844 Implementers' Call #1](https://www.youtube.com/watch?v=hAa1b4m7tsI&ab_channel=EthereumFoundation)
	- [Ethereum Core Devs Meeting [#148] [2022-10-27]](https://www.youtube.com/watch?v=oQfEW8LdE88&ab_channel=EthereumFoundation)  
- Starting to play around with Rust based crypto libraries (completing CE text exercises):
	- https://cryptography.rs/
	- https://github.com/rustls/rustls
	- https://github.com/MystenLabs/fastcrypto

### Friday
- Streamed ch 2.8 -> 3.5
	- organized all exercises (ch 1-3)
	- went through relevant exercises (AES in Rust with https://docs.rs/openssl/latest/openssl/aes/index.html#aes-ige)
- Project plan seems to be focusing around 3 main flows:
	1. Complete all core exercises in Rust
		- Find relevant/up to date Rust libs.
		- Create guides (written/video) for completing exercises OS on Github.
	2. Update material & exercises for modern cryptography engineering practices/knowledge.
		- Go through text and take notes while focusing on key areas that may be outdated.
		- This will require more knowledge than I current have, but mentor/Fellowship support should be beneficial.
	3. Cater examples to be practical and based on blockchain implementations. 
		- Generally providing relevant industry examples from the multitude of real projects building in the zk space.
		- Most efficient if these examples are community curated
- At the end of the 4 mo Fellowship, I hope to have this new resource available on the Uncloak site. 
	- similar to: https://cryptobook.nakov.com/?
- Going to spend the weekend browsing through the materials on the reading list a bit more ([mats from](https://github.com/ingonyama-zk/ingopedia))
	- https://www.youtube.com/playlist?list=PLj80z0cJm8QErn3akRcqvxUsyXWC81OGq
	- https://cryptobook.nakov.com/
	- https://decentralizedthoughts.github.io/2020-12-22-what-is-a-merkle-tree/
	- https://www.youtube.com/watch?v=fOGdb1CTu5c&ab_channel=WIRED
	- https://www.youtube.com/watch?v=iRQw2RpQAVc&ab_channel=LeastAuthority
	- https://www.youtube.com/channel/UC1usFRN4LCMcfIV7UjHNuQg/videos
	- https://www.youtube.com/playlist?list=PLJijNYoOwnssZzIIxfochRxo5QRW5Uvfg
	- https://www.win.tue.nl/~berry/CryptographicProtocols/LectureNotes.pdf
	- https://crypto.stanford.edu/cs355/22sp/schedule/
		- https://toc.cryptobook.us/book.pdf
		- https://crypto.stanford.edu/~dabo/papers/bfibe.pdf
	- https://www.youtube.com/watch?v=HO97kVMI3SE&ab_channel=a16zcrypto


