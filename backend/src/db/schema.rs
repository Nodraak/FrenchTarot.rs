table! {
    game_players (uuid) {
        uuid -> Text,
        user_uuid -> Nullable<Text>,
        game_uuid -> Nullable<Text>,
    }
}

table! {
    games (uuid) {
        uuid -> Text,
        max_players_count -> Integer,
        creator_uuid -> Nullable<Text>,
    }
}

table! {
    users (uuid) {
        uuid -> Text,
        username -> Text,
    }
}

joinable!(game_players -> games (game_uuid));
joinable!(game_players -> users (user_uuid));
joinable!(games -> users (creator_uuid));

allow_tables_to_appear_in_same_query!(
    game_players,
    games,
    users,
);
