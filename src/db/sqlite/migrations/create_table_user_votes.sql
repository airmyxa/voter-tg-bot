create table user_votes (
    id bigserial primary key,
    vote_id bigint references votes (id),
    user_name text not null,
    choice text not null,

    unique(vote_id, user_name)
);
