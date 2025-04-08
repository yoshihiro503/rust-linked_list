# Immutable Linked List

A simple **immutable linked list** implemented in pure Rust.  
Inspired by functional languages like OCaml and Haskell.

---

## ✨ Features

- Immutable, persistent data structure
- Functional style: `cons`, `from_vec`, `reverse`, and more
- Safe and idiomatic Rust
- No unsafe code, no external dependencies

---

## 📦 Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
linked_list = { path = "../rust-linked_list" }
```

## 🔧 Usage
```rust
use my_linked_list::List;

fn main() {
    let list = List::from_vec(vec![1, 2, 3]);
    let reversed = list.reverse();

    println!("{:?}", reversed); // Cons(3, Cons(2, Cons(1, Nil)))
}
```

## 📘 API Overview

```rust
pub enum List<T> {
    Nil,
    Cons(T, Box<List<T>>),
}

impl<T> List<T> {
    pub fn from_vec(vec: Vec<T>) -> Box<List<T>>;
    pub fn from_array(array: &[T]) -> Box<List<T>> where T: Clone;
    pub fn reverse(&self) -> Box<List<T>> where T: Clone;
    // ... and more
}
```

## 🧪 Tests

Run unit tests:
```rust
cargo test
```

## 📄 License

Licensed under:

* MIT License (LICENSE-MIT or http://opensource.org/licenses/MIT)

## 🤝 Contributing

Pull requests, issues, and suggestions are welcome!