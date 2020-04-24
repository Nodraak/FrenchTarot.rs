table! {
    game_players (pk) {
        pk -> Integer,
        user_pk -> Nullable<Integer>,
        game_pk -> Nullable<Integer>,
    }
}

table! {
    games (pk) {
        pk -> Integer,
        max_players_count -> Integer,
        creator_pk -> Nullable<Integer>,
    }
}

table! {
    users (pk) {
        pk -> Integer,
        username -> Text,
    }
}

joinable!(game_players -> games (game_pk));
joinable!(game_players -> users (user_pk));
joinable!(games -> users (creator_pk));

allow_tables_to_appear_in_same_query!(
    game_players,
    games,
    users,
);
