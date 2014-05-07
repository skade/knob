RUSTC ?= rustc
RUSTDOC ?= rustdoc
RUSTTEST ?= rustc --test
RUSTFLAGS ?= -O --out-dir build -L build

.SILENT:

all: libknob test
	## Build library and run tests

libknob: builddir
	# --
	$(RUSTC) $(RUSTFLAGS) src/knob/lib.rs

builddir:
	# --
	mkdir -p build

libknob-test: libknob
	# --
	$(RUSTTEST) $(RUSTFLAGS) src/knob/test.rs

test: libknob-test checkdocs
	## Run tests
	./build/test

clean:
	## Remove library, test files, documentation
	git clean -df
	rm -fr build/ doc/

checkdocs: libknob
	# Check documentation
	$(RUSTDOC) --test -L build README.md
	$(RUSTDOC) --test -L build src/knob/lib.rs

docs: checkdoc
	## Generate API documentation
	$(RUSTDOC) -o doc/ src/knob/lib.rs
