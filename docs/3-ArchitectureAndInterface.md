# Arch

* tarot-lib
    * Reuse as much as possible between server and client.
    * Client is a web client, but a cli or gui client could exist as well.
    * Client knows whose turn it is and whether an action is valid. Server still checks to prevent cheat.
* Data
    * Main data is functional data defined in tarot-lib. Client just use it as it is ; server has a `Vec<(client_uuid, client_data)>` to hold one for each client.
    * Data might be saved in a db later, to be accessed by http website.


# Interface

## State (data)

Game data depends on game state:

```rust
// cf tarot-lib/src/game/game.rs

// public data (shared among every player as it is)
struct GameState {
    max_players: i8,                            // used to auto start game
    players_username: Vec<Uuid, String>,        // by seating order, counter-clockwise
    state: GameState,
    active_player: Option<Uuid>,                // who are we waiting for, if any

    king: Option<CardSuit>,
}

// private data (player specific, details are hidden to non owner)
struct PlayerState {
    player_uuid: Uuid,
    hand: Option<CardsPile>,
    dog: Option<CardsPile>,
    scoring_pile: Option<CardsPile>,
}
```

## Events (transitions)

```rust
// cf tarot-lib/src/game/events.rs

enum Event {
    // websocket events
    WsConnect(WsConnectPayload),
    WsDisconnect(WsConnectPayload),

    // game data
    Game(GamePayload),

    // register as player
    GameJoin(WsConnectPayload),                    // on last player: transition WaitingPlayers -> DealingCards

    DealResult(DealResultPayload),                 // transition DealingCards -> Bidding

    // bids
    BidAnnounce(BidAnnouncePayload),
    BidResult(BidAnnouncePayload),                 // transition Bidding -> PreparingKing

    // preparing
    KingCalled(KingCalledPayload),                 // transition PreparingKing -> PreparingDog
    DogResult(DogResultPayload),                   // transition PreparingDog -> Playing

    PlayCard(PlayCardPayload),                     // on last card: transition Playing -> Finished
}
```
