# Quadratic Arithmetic Programs: from Zero to Hero
> 2016 [medium article by Vitalik Buterin](https://medium.com/@VitalikButerin/quadratic-arithmetic-programs-from-zero-to-hero-f6d558cea649) 

- Diagram drawn by zk-SNARK researcher Eran Tromer![](Pasted%20image%2020221108104618.png)
- The steps here can be broken up into two halves.
- First, zk-SNARKs cannot be applied to any computational problem directly.
	- You have to convert the problem into the right “form” for the problem to operate on.
	- The form is called a “quadratic arithmetic program” (QAP)
	- There is another process that can be run alongside QAP to create a witness.
	- Then, intricate process for creating zk-proof with witness. 
	- Then another process for verifying a proof that someone passes to you (out of scope for this article).
- Example will be a simple one: proving that you know solution to cubit eq: $x^3 + x + 5 == 35$ (ans: 3)
```python
def qeval(x):  
y = x**3  
return x + y + 5
```
- Special-purpose language that we use supports basic arithmetic $(+, -, *, /)$; no loops allowed (number of computational steps bounded).
-  Note that modulo (%) and comparison operators (<, >, ≤, ≥) are NOT supported.
	- No efficient way to do modulo or comparison directly in finite cyclic group arithmetic.
	- If there was a way to do either one, then elliptic curve cryptography would be broken.
	- You can extend the language to modulo and comparisons.
	- We can extend the language to support conditionals (eg. `if x < 5: y = 7; else: y = 9`) by converting them to an arithmetic form: `y = 7 * (x < 5) + 9 * (x >= 5)`.
- Edu purposes only [Python compiler for making QAPs](https://github.com/ethereum/research/tree/master/zksnark)

## Flattening
> The first step is a “flattening” procedure; convert the original code into a sequence of statements that are of two forms:`x = y` and `x = y (op) z` (where `op` can be +, -, *, / and `y` and `z` can be variables, numbers or themselves sub-expressions).

```python
sym_1 = x * x  
y = sym_1 * x  
sym_2 = y + x  
~out = sym_2 + 5
```

## Gates to R1CS
> A rank-1 constraint system (R1CS) is a sequence of groups of three vectors `(a, b, c)`, and the solution to an R1CS is a vector `s`, where `s` must satisfy the equation `s . a * s . b - s . c = 0`, where `.` represents the dot product.

- A satisfied R1CS:![](Pasted%20image%2020221108162931.png)
- We are going to have many constraints: one for each logic gate.
```python
'~one', 'x', '~out', 'sym_1', 'y', 'sym_2'
```

1. `(a, b, c)` triple for the first gate: `sym_1 = x * x`  
```python
a = [0, 1, 0, 0, 0, 0]  
b = [0, 1, 0, 0, 0, 0]  
c = [0, 0, 0, 1, 0, 0]
```
- The purpose of this first check is to verify the consistency of the inputs and outputs of the first gate only.

2. the second gate: `y = sym_1 * x`  
```python
a = [0, 0, 0, 1, 0, 0]  
b = [0, 1, 0, 0, 0, 0]  
c = [0, 0, 0, 0, 1, 0]
```
- Here we’re checking that `sym_1 * x = y`.

3. The third gate: `sym_2 = y + x`
```python
a = [0, 1, 0, 0, 1, 0]  
b = [1, 0, 0, 0, 0, 0]  
c = [0, 0, 0, 0, 0, 1]
```
- Addition check; checking that all output equals the sum of the two inputs. 

4. Finally, the fourth gate: `~out = sym_2 + 5`
```python
a = [5, 0, 0, 0, 0, 1]  
b = [1, 0, 0, 0, 0, 0]  
c = [0, 0, 1, 0, 0, 0]
```
- Here, we’re evaluating the last check, `~out = sym_2 + 5`.
- Dot product check.

The witness is simple the simply the assignment to all the variables, including input, output and internal variables:

```python
[1, 3, 35, 9, 27, 30]
```

The complete R1CS put together is:

```python
A  
[0, 1, 0, 0, 0, 0]  
[0, 0, 0, 1, 0, 0]  
[0, 1, 0, 0, 1, 0]  
[5, 0, 0, 0, 0, 1]
B  
[0, 1, 0, 0, 0, 0]  
[0, 1, 0, 0, 0, 0]  
[1, 0, 0, 0, 0, 0]  
[1, 0, 0, 0, 0, 0]
C  
[0, 0, 0, 1, 0, 0]  
[0, 0, 0, 0, 1, 0]  
[0, 0, 0, 0, 0, 1]  
[0, 0, 1, 0, 0, 0]
```

## R1CS to QAP
> 