# Reqs

## MVP

* Functional
    * Play [tarot](https://en.wikipedia.org/wiki/French_Tarot)
    * Play with players and bots
    * Soft/hot (re)load: refresh mid game
* Tech
    * Full Rust, back and front (no js)
    * Classic http server-client for main website ; wasm and websocket for game

## Improvements

* Show created/on going games on welcome screen
* Hard/cold (re)load: keep game state even when all players disconnect
* A player can play several games at the same time
* Keep some kind of history/highscore
* Create a game and play several "manches"
