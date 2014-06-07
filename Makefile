RUSTC ?= rustc
RUSTDOC ?= rustdoc
RUSTTEST ?= rustc --test
RUSTFLAGS ?= -O --out-dir build -L build

.SILENT:

all: libknob test examples
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
	./build/test

test: libknob-test checkdocs

clean:
	## Remove library, test files, documentation
	git clean -df
	rm -fr build/ doc/

examples: libknob
	$(RUSTC) -L build --out-dir examples/ examples/commandline_options.rs
	$(RUSTC) -L build --out-dir examples/ examples/socket_settings.rs
	
checkdocs: libknob
	# Check documentation
	$(RUSTDOC) --test -L build src/knob/lib.rs
	$(RUSTDOC) --test -L build README.md

docs: checkdocs
	## Generate API documentation
	$(RUSTDOC) -o doc/ src/knob/lib.rs
	$(RUSTDOC) -o doc/ README.md
	$(RUSTDOC) -o doc/ CHANGES.md
	$(RUSTDOC) -o doc/ doc/index.md
