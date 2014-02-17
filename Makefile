RUSTC ?= rustc
RUSTTEST ?= rustc --test
RUSTFLAGS ?= -O --out-dir build -L build

libknob:
	$(RUSTC) $(RUSTFLAGS) src/knob/lib.rs

libknob-test:
	$(RUSTTEST) $(RUSTFLAGS) src/knob/test.rs

test: libknob-test
	./build/test
