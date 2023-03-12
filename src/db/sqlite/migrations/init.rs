pub static INIT_DB: &'static str = "
create table votes (
    id bigserial primary key,
    chat_id text not null,
    message_id text not null,
    user_name text not null,
    query_data text not null,
    
    unique(chat_id, message_id, user_name)
);
";
