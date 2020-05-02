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
