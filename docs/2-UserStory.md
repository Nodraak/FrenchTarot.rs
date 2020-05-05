# User Story


```
        S                       C1                      C2                      C3
db      http    ws          http    ws             http    ws             http    ws

// first connection, create game
          <- /game/play/<id> --
          --- wasm ----------->
// GameData for player + Connect for all
                 --- GameData ------>
                 --- broadcast Connect
                 --- broadcast GameJoin  // for now, auto join as player, will allow guest to watch game later

// second player
                 <--------------- /game/play/<id> --
                 --- wasm ------------------------->

// GameData for player + Connect for all
                 --- GameData ---------------------------->
                 --- broadcast Connect
                 --- broadcast GameJoin  // auto join

// third player
                 <---------------------------------------- /game/play/<id> --
                 --- wasm -------------------------------------------------->

// GameData for player + Connect for all
                 --- GameData ----------------------------------------------------->
                 --- broadcast Connect
                 --- broadcast GameJoin  // auto join

// all players are here, state changes to DealingCards
                 --- broadcast DealResult  // for now, auto deal

// stage changes to Bidding
                 <-- BidAnnounce ----- // each player sends his bid to the server, which broadcast it to everyone
                 // broadcast BidAnnounce
// when all player have bidded, server sends BidResult
                 --- broadcast BidResult

// state changes to Preparing
                 // leader sends KingCalled then DogResult
                 // broadcast KingCalled
                 // broadcast DogResult

// state changes to Playing
                 // players send PlayCard

// on last card, state changes to Finished
                 // broadcast message with scores

```
