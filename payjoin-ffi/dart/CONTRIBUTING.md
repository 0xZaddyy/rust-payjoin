# Contributing

Instructions for building and testing the Dart bindings locally.

## Build Bindings

Follow these steps to clone the repository and run the tests.
This assumes you already have Rust and Dart installed.

```shell
git clone https://github.com/payjoin/rust-payjoin.git
cd rust-payjoin/payjoin-ffi/dart

# Install package dependencies
dart pub get

# Generate the bindings
bash ./scripts/generate_bindings.sh
```

## Running Tests

```shell
# Run all tests
dart test
```
