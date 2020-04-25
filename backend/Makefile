help:
	echo "Help"

build:
	make -C ../tarot-game build
	rm -rf static/bin/ \
		&& mkdir static/bin/ \
		&& cp ../tarot-game/pkg/tarot_game.js ../tarot-game/pkg/tarot_game_bg.wasm static/bin/
	cargo build

run:
	echo "DATABASE_URL=db.sqlite3" > .env
	cargo run
