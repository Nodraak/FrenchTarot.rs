help:
	echo "Help"

build:
	cargo build

run:
	rm -rf static/bin/ \
		&& mkdir static/bin/ \
		&& cp ../tarot-game/pkg/tarot_game.js ../tarot-game/pkg/tarot_game_bg.wasm static/bin/
	cargo run
