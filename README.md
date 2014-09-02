# Rust leveldb bindings

Incomplete bindings for leveldb for Rust.

## Prerequisites

`snappy` and `leveldb` need to be installed. On Ubuntu, I recomment:

```sh
sudo apt-get install libleveldb-dev libsnappy-dev
```

## Usage

If your project is using [Cargo](http://crates.io), drop the following lines in your `Cargo.toml`:

```
[dependencies.leveldb]

git = "https://github.com/skade/leveldb.git"
```

## Development

Make sure you have all prerequisites installed. Run

```
$ cargo build
```

for building and

```
$ cargo test
```

to run the test suite.

## TODOS

* Remove a lot of warnings
* Implement the whole interface

# License

MIT, see `LICENSE`
