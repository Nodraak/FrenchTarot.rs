create table users (
    uuid                varchar         not null        primary key,
    username            varchar         not null
);

create table games (
    uuid                varchar         not null        primary key,
    max_players_count   integer         not null,
    creator_uuid        varchar,
    foreign key(creator_uuid) references users(uuid) on delete set null
);
create table game_players (
    uuid                varchar         not null        primary key ,
    user_uuid           varchar,
    game_uuid           varchar,
    foreign key(user_uuid) references users(uuid) on delete set null
    foreign key(game_uuid) references games(uuid) on delete set null
);
