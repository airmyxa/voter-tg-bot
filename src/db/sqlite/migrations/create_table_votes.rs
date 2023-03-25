pub static CREATE_TABLE_VOTES: &'static str = "
create table votes (
   id integer primary key autoincrement,
   chat_id text not null,
   message_id text not null,
   text text not null,

   unique(chat_id, message_id)
);
";
