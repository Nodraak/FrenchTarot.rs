# tarot.rs

French tarot game built with Rust and WASM.

## Architecture

* Tarot-lib: used by Tarot-game and backend. Contains Card, Player, Game, etc. Lib.
* Tarot-game: tarot client. WASM app.
* Backend: tarot server. Exe.

Tarot-game and Backend communicate with HTTP post requests. Tarot-game makes
regular statys check requests and backend answers with a list of Messages. Yes,
it should be a websocket or something, but let's make a working MVP first.
