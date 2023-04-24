create table votes
(
    id          integer primary key autoincrement,
    chat_id     text   not null,
    message_id  text   not null,
    text        text   not null,
    template_id bigint not null,

    unique (chat_id, message_id)
);
