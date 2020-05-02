help:
	echo "Help"

all: build run

build:
	make -C tarot-lib/ build
	make -C tarot-client/ build
	make -C tarot-server/ build

run:
	make -C tarot-server/ run

test:
	make -C tarot-lib/ test
	make -C tarot-client/ test
	make -C tarot-server/ test

doc:
	cd tarot-client/ && cargo doc --no-deps --all-features --document-private-items --target-dir ../target/
	cd tarot-server/ && cargo doc --no-deps --all-features --document-private-items --target-dir ../target/
