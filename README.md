Postfixer
=========

## Why a Postfixer

There's multiple ways to evaluate expressions but how does a postfixer work?

Let's say we have an expression like `A + B * C`, we as humans have learned that we first do `B * C` then add `A` but how does the computer knows? That's where the postfixe expression comes in.

It transforms our algebric expression as a special expression that can be evaluated more easily by the computer like:

- `A + B * C` $\rightarrow$ `A B C * +`

It evaluates the operator with the two operands imediately at the left of the operator itself

## How To Use

Clone this repository:

```bash
git clone https://github.com/hectellian/postfixe.git
```

Compile the binary file


```bash
cargo build --release
```
Then, in the command line juste type:

```bash
./target/release/postfixe <arithmetic expression>
```

---

Or (if you already hace the binary file)

```bash
./postfixe "arithmetic expression"
```