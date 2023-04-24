create table user_votes
(
    id        integer primary key autoincrement,
    vote_id   bigint references votes (id),
    user_name text not null,
    choice    text not null,

    unique (vote_id, user_name)
);
