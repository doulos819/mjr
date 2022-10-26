##### An Opinionated Introduction to backend Development (Luca Palmieri)

### Preface
- Z2P will focus on the challenges of writing Cloud-native applications in a team of four or five engineers with different levels of experience and proficiency.
- *Cloud-native app*: 
	- Achieve high-availability while running in fault-prone environments.
	- Allow to continually release new versions with no downtime.
	- Handle dynamic workloads/request volumes.
	- *Distributed*: multiple instances of the app running on multiple machines. 
- Book will cover:
	- How to instrument your Rust app to obverse system. 
	- How to set up and evolve database schema via migrations.
	- Cover all material required to use Rust to tackle both day one and day to concerns of a Cloud native API.
- Main focus is *enterprise software*: correct code which is expressive enough to model the domain and supple enough to support its evolution over time.
	-  Get it to run first, optimize it later if needed.
- Can Rust be a productive language for API development?
	- Yes :P (but can take some time to figure out how)
- Book is meant for seasoned backend devs who have read The Rust Books and are trying to port over a couple of simple systems. 
  
  ### Chapter 1 - Getting started
  > A brief tour of a set of tools and utilities that are going to be useful in the journey.
  
  #### 1.1 Installing The Rust Toolchain
  > There are various ways to [install Rust](https://www.rust-lang.org/tools/install)
  
  rustup is more than a Rust installer - its main value is *toolchain management*
	- *toolchain*: the combination of a compilation target and a release channel.
##### 1.1.1 Compilation Targets
  



