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
	make -C tarot-client/ doc
	make -C tarot-server/ doc
