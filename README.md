# unabbreviated-rust

Some weirdo wanted Rust keywords unabbreviated
and a couple other deranged changes like renaming `let` to `var`...

I do not condone what this crate does.
Just use Rust.

Based on [rouille](https://github.com/bnjbvr/rouille).

## Usage

If you for some reason want this:

`Cargo.toml`:

```toml
unabbreviated = "0.1"
```

Usage:

```rust
unabbreviated::rust! {

public function main() {
    println!("Hello, world!");
}

// ... your code here
}
```

## Usage but cooler

If only Rust applied the macro during parsing and not after 🥲 .
Yeah, unfortunately **this doesn't actually work**.

`main.rs`/`lib.rs`:

```rust
// Tell the compiler you have no fear
// This is only needed once in `main.rs`/`lib.rs`
#![feature(proc_macro_hygiene)]
#![feature(custom_inner_attributes)]
// Requires Rust nightly, as all cool things do
```

Other `.rs` files:

```rust
// Put this macro at the top of each file
#![unabbreviated::file]

public function greet() {
    println!("Hello, world!");
}

// ... your code here
```

`rust-toolchain.toml`:

```toml
[toolchain]
channel = "nightly"
```

See also:

- [rust#54726](https://github.com/rust-lang/rust/issues/54726) Tracking issue for custom inner attributes
- [rust#54727](https://github.com/rust-lang/rust/issues/54727) Tracking issue for procedural macros and "hygiene 2.0"

## Future plans

Patch `rustc`? Nah... [Unless](https://dev.to/xphoniex/adding-our-own-custom-statement-to-rust-language-30lc)?
