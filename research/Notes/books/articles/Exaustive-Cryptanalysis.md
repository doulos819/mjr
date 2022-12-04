#  Exhaustive Cryptanalysis of the NBS DES
> Whitfield Diffie and Martin E. Hellman Stanford University

## Introduction
> The growing commercial need for cryptographic systems, however, requires an expansion of public knowledge in this area.

- The following section provides the basic argument concerning the standard's inadequate level of security. It shows that, using the simplest of cryptanalytic attacks, a $20 million machine can be built to break the proposed standard in about 12 hours of computation time.
- More seriously, in about 10 years time, the rapidly decreasing cost of computation will bring the machine's cost- down to the $200,000 range, and the cost per More seriously, in about 10 years time, the rapidly decreasing cost of computation will bring the machine's cost- down to the $200,000 range, and the cost per
- While NBS disputes our claims, it has indicated that changing technology will probably cause the standard to be revised in five or 10 years.'

## The basic argument
- The proposed standard transforms a block of 64 plain-text (unenciphered) bits, denoted P, into a block of 64ciphertext bits C. This transformation is governed by a 56-bit keyK, and is invertible so that
	- $C = S_K(P)$
	- $P = S_K^{-1} (C)$ 
	- where $S_K$ is the enciphering transformation when key $K$ is used. There are $2^{56}$ keys.
- We will consider a known-plaintext cryptanalytic attack in which the cryptanalyst has several corresponding plaintext-ciphertext blocks, all encrypted in the same key. Find $K$.
- This planned obsolescence is unwarranted, and it is easily remedied by increasing the key length from 56 bits to 128 or 256 bits. Use of a 128-bit key would increase the estimated cost for a brute force search from $5000 to  $2 X 10^{25}$, and no foreseeable technological advances would allow this to be brought into a reasonable range. 
- The authors, with several others, conducted a short(one month) study" which looked for structure and potential weaknesses in the standard. A symmetry under complementation was found which allows a 50% savings in search effort under a chosen plaintext attack.
- This study suffered because the government has askedIBM to keep secret the structures designed into the memory tables used by the Data Encryption Standard(DES). 
- We think it is unreasonable for a public security standard to have secret structures because, if someone involved in the design of the standard were to turn against it, he would be in a much better position to break it. For this reason, it is a well-established principle that the security of the crypto system should not depend on secret design principles.

## Objections to the basic argument

Design and control costs - Objection: The design and control costs overshadow theCPU hardware costs in a parallel processor. These costs grow much faster than linearly in the degree of parallelism.
- The repetitive structure, low I/O volume, and lack of component interaction greatly simplify the design, including automatic fault diagnosis.
- Our conclusion is that, today, design and control costs will not greatly add to the total cost.

MTBF - Objection: The mean time between failures in a million-component system would be much less than one day.The machine would hardly ever complete an error-free search.
- By cooling the machine room it should be possible to obtain the 0.05%-per-1000-hour failure rate which would correspond to a two-hourMTBF.
- The machine would be built in 64 racks, each with about 16,000 components. Each rack would be capable of detecting its own chip failures and of signalling a minicomputer controller to switch over to a spare rack while the fault was being repaired.

Speed and cost - Objection: It is not possible to build an LSI chip to test a key in 1 $\mu$sec for $10. Rather, 40 $\mu$sec and $100 are needed.

Physical size Objection: a million-chip machine would require 6000 six-foot-high racks.

Power requirements Objection: A million-chip device operating at high speed would consume too much power.

Changing keys Objection: Cryptanalysis by exhaustive search can be nullified by frequently changing keys.
- 

Uniqueness of solution Objection: There is no guarantee that a single plaintextciphertext pair will uniquely determine the key.

Our conclusion is that none of these objections are really valid, and that the 56-bit key is too small for a national standard cryptosystem.

## System architecture
- To avoid MTBF problems and to minimize design and control costs, the machine has 64 search racks, operating almost autonomously, all under the control of a minicomputer. There are three or four spare racks to provide backup for failed units which are being repaired.
- Automatic fault diagnosis is built in by having the board controller check any solutions offered it. This is done by having an extra search chip on each board.
- These checks will automatically diagnose and correct a fault which causes a false alarm, but there is a need to detect and diagnose a fault which would cause a valid solution to be lost. This is accomplished by having each rack go into a check mode every 30 minutes. Each chip should produce the same solution when polled. 

## Chip design
- The power, gate density, and speed requirements point to CMOS/SOS or N-channel depletion load as the technology to be used
- A density of at least 100 gates/mm^2 is needed. Also, a gate delay of about 4 nsec is needed to achieve a search time of 1 sec per key.
- Aside from being well within the order of magnitude estimate that is needed, the speed-power product for a static device is higher than for devices which are constantly processing. The exact savings depends on particulars of the chip layout, but savings of as much as 90% (to 0.2 pJ) might be possible.
- And if all 64 key bits were used in the algorithm, users would have the option of forsaking an internal parity check for a PERMUTATION P EXPANSION higher level of security. A true 64-bit key would increase the cost of exhaustive search by a factor of 28 from $5000 to $1,000,000. Since permuted choice one has (32 negative cryptographic value, we recommend it be eliminated.
- Following this initial permutation, the remaining 56 -I RREGISTER key bits are loaded into two 28-bit shift registers labeled C and D.
- The number of CIPHERTEXT REGISTER PLAINTEXT REGISTER left shifts (one or two) at each iteration is a fixed part of the algorithm which we call the shift schedule.
- They are described in the Federal Register' and in the document, "FIPS PUB 46, Data Encryption Standard," available from the National Technical Information Service, 5285 Port Royal Road, Springfield, VA 22161, for $3.50.
- instead of incrementing the key counter after checking whether the computed and known plaintexts match, this is done during the 16 iterations and check. Similarly, the C and D shift schedule (SS) registers are shifted during the portion of each iteration when their contents are not needed.
- It is seen that 6400 devices is a reasonable estimate. This is comparable to the complexity of a Z-80 microprocessor and thus requires a high-density technology.
- An XOR operation involves two gate delays or 6 nsec. The ROM access takes about seven gate delays or 21 nsec. The second XOR also takes 6 nsec. And loading the L and R registers takes about four gate delays (12 nsec). The total time for one iteration is thus 6 + 21 + 6 + 12 nsec = 45 nsec, and 16 iterations takes 720 nsec. Allowing 80 nsec for loading C, D, L, and R and for the check (see Figure 5) brings the total time to test one key to 800 nsec, or slightly less than the 1 ,usec used in the basic argument.
- Even using the extremely conservative figure of 10 nsec/gate delay would result in a search time of less than 3 ,usec per key, - 15k$

## Ciphertext-only attack
- The cryptosystem is a FIPS (Federal Information Processing Standard) standard, binding on all applicable federal agencies. Because ASCII is also a FIPS standard, the plaintext data will often by represented as 8-bit ASCII characters.
- the ciphertext-only attack the chip is given two ciphertext blocks and its starting point in the key space. Upon entering the search mode the chip deciphers the first ciphertext block and checks if the eight parity positions in the computed plaintext block are valid parity bits. If the key tried was not the right key, there is one chance in 28 = 256 of these parity bits appearing valid.
- Very little penalty is paid for modifying the machine to handle both known plaintext and ciphertext-only attacks. The number of gates added to the chip is about 200 for 64 XOR's, and the search rate is essentially unchanged since only one out of 256 keys has a doubled search time on the chip. The extra hardware in the board and rack control units is also minimal.

## Variable key-size techniques
- a standard with a variable key size is highly desirable. This section describes only two of the many ways to achieve this goal.
- First the key, followed by enough zeros to make a total of 128 bits, is used to drive a 128-bit linear feedback shift register.
	- S. Golomb, Shift Register Sequences, Holden Day, San Francisco, 1967.
- The good psuedo-random properties of feedback shift register sequences ensure a good distribution within the final shift register contents even if a short key is used. After this initial set-up, we have 128 keying bits regardless of the original key size. The feedback connections are disabled and the shift register is used to shift its final contents cyclically eight positions after each of the 16 iterations of the basic encryption loop; 48 of the 128 stages are tapped to provide the 48 keying bits needed for each iteration. After the sixteenth iteration, the 128-bit shift register has returned to its initial position and is ready to encipher the next block. By reversing the direction of the shifts, the keying bits -are obtained in reverse order for deciphering.

## Discussion
- It is now possible to see why the search machine should benefit fully from falling computation costs. Since the chip is compute bound, not I/O limited, it will receive full benefit from increases in gate speed. Also, as greater gate densities or larger chips become possible, many search devices can be put on one chip with a multiplexer to keep the pin count unchanged.
- A 128-bit or larger key is needed to preclude exhaustive search and to allow a margin of safety against shortcuts to exhaustive search. We hope that those readers who will be most affected by this standard will let their views be known to NBS.