CARGO := cargo

DEBUG := 0
ifeq ($(DEBUG), 0)
	CARGO_OPTIONS := --release --locked
else
	CARGO_OPTIONS :=
endif

.PHONY: all aoc2022 test lint clean

all: aoc2022 test lint

aoc2022:
	$(CARGO) build $(CARGO_OPTIONS)

test:
	$(CARGO) test $(CARGO_OPTIONS)
lint:
	$(CARGO) fmt -- --check
	$(CARGO) check
	$(CARGO) clippy --all -- -D warnings
	$(CARGO) audit

clean:
	$(CARGO) clean
