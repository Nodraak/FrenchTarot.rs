help:
	echo "Help"

all: build run

build:
	make -C tarot-lib/ build
	make -C tarot-game/ build
	make -C backend/ build

run:
	make -C backend/ run
