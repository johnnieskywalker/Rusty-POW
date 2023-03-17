# Rusty PoW Blockchain

A simple implementation of a Proof of Work (PoW) blockchain in Rust, discovered on a rusted computer inside a rusted FSO Polonez Cargo.

## The Story

Legend has it that the creator of this basic Proof of Work blockchain, known as Rusty PoW, stored the source code on a rusted computer. This computer was found hidden in the trunk of an abandoned, rusted FSO Polonez Cargo. The car had been left in a forgotten corner of a junkyard for years, until one day, an adventurous programmer stumbled upon the treasure.

Now, Rusty PoW has been brought to life for the world to witness and use. This simple but powerful blockchain implementation showcases the power of Rust and the ingenuity of its creator.

## Overview

This project demonstrates a basic blockchain with a Proof of Work consensus algorithm. It includes the following features:

- Adding transactions to a mempool
- Mining blocks with a configurable difficulty target
- A basic test suite to ensure the blockchain functions as expected

## Prerequisites

To run this project, you'll need to have Rust installed. You can download Rust [here](https://www.rust-lang.org/tools/install) and follow the installation instructions for your operating system.

## Getting Started

1. Clone the repository:

   ```sh
   git clone git@github.com:johnnieskywalker/Rusty-POW.git
   cd rusty_pow_blockchain
   ```
2. Build the project

```sh
cargo build
```

3. Run the tests

```sh
cargo test
```

# Usage
To use the rusty_pow_blockchain library in your Rust project, add the following to your Cargo.toml:

```sh
[dependencies]
rusty_pow_blockchain = { path = "/path/to/rusty_pow_blockchain" }
```
Then, you can import and use the rusty_pow_blockchain library in your code:

```rust
use rusty_pow_blockchain::{Blockchain, Transaction};

fn main() {
    let blockchain = Blockchain::new();
    
    // Add transactions to the mempool
    blockchain.add_transaction("Alice".to_string(), "Bob".to_string());
    blockchain.add_transaction("Bob".to_string(), "Charlie".to_string());

    // Mine a block
    blockchain.mine();

    // View the current state of the blockchain
    let blocks = blockchain.blocks();
    let mempool = blockchain.mempool();
}
```
