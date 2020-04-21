help:
	echo "Help"

all: build run

build:
	make -C tarot-lib/ build
	make -C tarot-game/ build
	make -C backend/ build

run:
	rm -rf backend/bin/ && mkdir backend/bin/
	cp tarot-game/pkg/tarot_game.js tarot-game/pkg/tarot_game_bg.wasm backend/bin/
	python3 -m http.server --directory backend/
