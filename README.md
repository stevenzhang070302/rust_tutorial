# 🌟 Rust Tutorials: Concepts and Examples

Welcome to my **Rust learning repository**! 🎉 This repository showcases my journey of learning Rust, demonstrating concepts and key topics through practice and exploration. Each folder contains code and resources related to a specific topic, making this a helpful reference for Rust beginners and enthusiasts alike.

---

## 📂 Table of Contents

1. [🔧 Primitive Data Types](#-primitive-data-types)
2. [🧰 Compound Data Types](#-compound-data-types)
3. [🛠️ Functions](#-functions)
4. [🔑 Ownership](#-ownership)
5. [🤝 Borrowing](#-borrowing)
6. [🔤 Variables](#-variables)
7. [🔢 Constants](#-constants)
8. [🔄 Shadowing](#-shadowing)
9. [🧠 Control Flow](#-control-flow)
10. [🔁 Loops](#-loops)
11. [🏗️ Structs](#-structs)
12. [🎭 Enums](#-enums)
13. [⚠️ Error Handling](#-error-handling)
14. [📦 Collections](#-collections)

---

### 🔧 Primitive Data Types

**Key Points:**
- Includes scalar types (integers, floats, booleans, and characters).
- Offers robust type safety and size specifications for numbers.
- Supports Unicode characters for comprehensive text representation.

---

### 🧰 Compound Data Types

**Key Points:**
- Tuples group multiple values of different types into one entity.
- Arrays store fixed-size collections of elements of the same type.
- Both provide foundational grouping capabilities for Rust programs.

---

### 🛠️ Functions

**Key Points:**
- Functions are defined using the `fn` keyword and can have parameters and return types.
- Encourages modular design by allowing reusable blocks of code.
- Return values are implicit, based on the last expression in the function.

---

### 🔑 Ownership

**Key Points:**
- Ownership is Rust's memory management system without a garbage collector.
- Ensures that each value has a single owner at any given time.
- Transfer of ownership (move semantics) and borrowing make programs memory-safe.

---

### 🤝 Borrowing

**Key Points:**
- Borrowing allows you to reference data without taking ownership.
- Immutable and mutable borrowing ensure safe concurrency and memory usage.
- References must follow Rust's rules for lifetimes to avoid dangling pointers.

---

### 🔤 Variables

**Key Points:**
- Variables in Rust are immutable by default.
- You can declare variables using `let` and make them mutable using `mut`.
- Rust encourages explicit type annotations but can infer types automatically.

---

### 🔢 Constants

**Key Points:**
- Constants are immutable values that must be explicitly typed.
- Declared using `const` and are often used for fixed values throughout the program.
- Constants are evaluated at compile time and cannot be changed.

---

### 🔄 Shadowing

**Key Points:**
- Shadowing allows you to redefine a variable within the same scope.
- Useful for transformations or type changes without making the variable mutable.
- Helps maintain immutability while still allowing variable updates.

---

### 🧠 Control Flow

**Key Points:**
- Rust supports standard control structures like `if`, `else`, and `match`.
- Loops include `loop`, `while`, and `for`, catering to various repetition needs.
- Pattern matching through `match` provides exhaustive and clean branching.

---

### 🔁 Loops

**Key Points:**
- Infinite loops with `loop` are the foundation of controlled repetition.
- `while` loops repeat as long as a condition evaluates to true.
- `for` loops iterate over a range or collection, providing safety and simplicity.

---

### 🏗️ Structs

**Key Points:**
- Structs group related data into a custom type.
- Can be either named (traditional) or tuple-like for lightweight use.
- Provides the foundation for object-oriented programming in Rust.

---

### 🎭 Enums

**Key Points:**
- Enums define a type with multiple distinct variants.
- Useful for modeling data that can take on a variety of states or forms.
- Variants can hold additional data, allowing powerful customization.

---

### ⚠️ Error Handling

**Key Points:**
- Rust handles errors using `Result` for recoverable and `panic!` for unrecoverable errors.
- Encourages handling errors explicitly to improve program robustness.
- The `Option` type models values that can be `Some` or `None`.

---

### 📦 Collections

**Key Points:**
- Dynamic collections include vectors, strings, and hash maps.
- Vectors are resizable arrays, while hash maps store key-value pairs.
- Collections are part of Rust’s standard library for flexible data manipulation.

---

## 🚀 How to Run the Code

1. Clone this repository:
   ```bash
   git clone https://github.com/yourusername/rust-tutorials.git
   cd rust-tutorials
