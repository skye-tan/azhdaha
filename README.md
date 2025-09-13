
# Azhdaha

![CI Status](https://github.com/skye-tan/azhdaha/actions/workflows/ci.yml/badge.svg)
![License](https://img.shields.io/github/license/skye-tan/azhdaha?style=flat-square)

Azhdaha is a compiler frontend for C that applies a linear type system to track memory ownership and detect issues like memory leaks, use-after-free, double frees, and dangling pointers at compile time. It aims to bring stronger memory safety guarantees to C through static analysis.

## How to Run

Make sure you have "cargo" and "just" installed.
To install "just" run:

```sh
cargo install just
```

Then you can use the provided recipes:

```sh
# Deploy the annotated libraries.
just deploy

# Build and run the analyzer.
just run --help
```

## Tests and Examples 

To run the examples:

```sh
just run-examples
```

To run the tests:

```sh
cd tests && python3 run-tests.py
```
