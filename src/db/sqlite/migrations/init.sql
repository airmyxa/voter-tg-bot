create table votes (
    id bigserial primary key,
    chat_id text not null,
    message_id text not null,
    user_name text not null,
    query_data text not null
);

create unique index uix_votes_chat_id_message_id_user_name on votes (chat_id, message_id, user_name);