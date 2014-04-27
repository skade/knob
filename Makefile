RUSTC ?= rustc
RUSTTEST ?= rustc --test
RUSTFLAGS ?= -O --out-dir build -L build

libknob: builddir
	$(RUSTC) $(RUSTFLAGS) src/knob/lib.rs

builddir:
	mkdir -p build

libknob-test: libknob
	$(RUSTTEST) $(RUSTFLAGS) src/knob/test.rs

test: libknob-test
	./build/test

clean:
	git clean -df
