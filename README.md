# unisecs [![Build Status](https://travis-ci.org/softprops/unisecs.svg?branch=master)](https://travis-ci.org/softprops/unisecs) [![Coverage Status](https://coveralls.io/repos/github/softprops/unisecs/badge.svg)](https://coveralls.io/github/softprops/unisecs) [![Software License](https://img.shields.io/badge/license-MIT-brightgreen.svg)](LICENSE) [![crates.io](https://img.shields.io/crates/v/unisecs.svg)](https://crates.io/crates/unisecs) [![Released API docs](https://docs.rs/unisecs/badge.svg)](http://docs.rs/unisecs) [![Master API docs](https://img.shields.io/badge/docs-master-green.svg)](https://softprops.github.io/unisecs)

> Unix epoch time representation that anyone can wear

## 🤔 About

Why not [`std::time`](https://doc.rust-lang.org/std/time/index.html)? Rust's `std::time` package provides two representations of time `SystemTime` and `Instant`. Often times you will need to work with an api that requires specifically [unix time](https://en.wikipedia.org/wiki/Unix_time) which typically is represented in terms of seconds. `std::time` representations are general purpose can can be made to represent unix time but not in a very straightforward or ergonomic way.

This crate does focuses specifically on that in addition to represented subsecond time for the purposes of capturing a more accuate measurement of a duration.

Unix seconds is a type of duration, anchored from a starting point of `00:00:00 UTC Thursday, 1 January 1970`. On most unix-based systems you can get this time on the command line with `date +%s`. This crate aims to provide similiar convenience in addition to having good interop with other features in `std::time` module.

## 📦 Install

In your Cargo.toml file, add the following under the [dependencies] heading

```toml
unisecs = "0.1"
```

## Usage

```rust
fn main() {
  println!(
    "{}", unixsecs::Seconds::now()
  );
}
```

Doug Tangren (softprops) 2019