create table votes (
   id bigserial primary key,
   chat_id text not null,
   message_id text not null,
   text text not null,

   unique(chat_id, message_id)
);
