create table users (
    pk                  integer primary key not null,
    username            varchar             not null
);

create table games (
    pk                  integer primary key not null,
    max_players_count   integer             not null,
    creator_pk          integer,
    foreign key(creator_pk) references users(pk) on delete set null
);
create table game_players (
    pk                  integer primary key not null,
    user_pk             integer,
    game_pk             integer,
    foreign key(user_pk) references users(pk) on delete set null
    foreign key(game_pk) references games(pk) on delete set null
);
