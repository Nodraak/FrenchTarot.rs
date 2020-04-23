help:
	echo "Help"

build:
	cargo build

run:
	rm -rf static/bin/ \
		&& mkdir static/bin/ \
		&& cp ../tarot-game/pkg/tarot_game.js ../tarot-game/pkg/tarot_game_bg.wasm static/bin/
	echo "DATABASE_URL=db.sqlite3" > .env
	cargo run
