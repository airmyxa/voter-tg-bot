/*
?1 - chat_id
?2 - message_id
?3 - user_name
?4 - query_data
 */

pub static UPSERT_USER_VOTES: &'static str = "\
insert into votes (chat_id, message_id, user_name, query_data) \
values (?1, ?2, ?3, ?4) \
on conflict (chat_id, message_id, user_name) do update \
set \
query_data = ?4;\
";
