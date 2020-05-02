help:
	echo "Help"

build:
	make -C ../tarot-client build
	rm -rf static/bin/ \
		&& mkdir static/bin/ \
		&& cp ../tarot-client/pkg/tarot_client.js ../tarot-client/pkg/tarot_client_bg.wasm static/bin/
	cargo build

run:
	echo "DATABASE_URL=db.sqlite3" > .env
	cargo run

test:
	cargo test

doc:
	cargo doc
