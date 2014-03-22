RUST ?= rust
RUSTC ?= rustc
RUSTTEST ?= rustc --test
RUSTFLAGS ?= -O --out-dir build -L build -L leveldb -C link-args="-lleveldb"
VERSION=0.1-pre

libleveldb: leveldb/libleveldb.dylib
	mkdir -p build/
	$(RUSTC) $(RUSTFLAGS) src/libleveldb/lib.rs

test: leveldb/libleveldb.dylib
	mkdir -p build/
	$(RUSTC) $(RUSTFLAGS) --test src/libleveldb/lib.rs
	build/leveldb

leveldb/libleveldb.dylib:
	cd leveldb
	make
	cd ..
