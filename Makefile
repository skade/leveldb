RUST ?= rust
RUSTC ?= rustc
RUSTTEST ?= rustc --test
RUSTFLAGS ?= -O --out-dir build -L build -C link-args="-lleveldb"
VERSION=0.1-pre

libleveldb:
	mkdir -p build/
	$(RUSTC) $(RUSTFLAGS) src/lib.rs

test: libleveldb
	mkdir -p build/
	$(RUSTC) $(RUSTFLAGS) --test src/test.rs
	rm -rf testdbs
	mkdir testdbs
	build/test
