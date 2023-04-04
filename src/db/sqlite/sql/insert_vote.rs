/*
?1 - chat_id
?2 - message_id
?3 - text
 */

pub static INSERT_VOTE: &'static str = "\
insert into votes (chat_id, message_id, text) \
values (?1, ?2, ?3) \
on conflict do nothing;\
";
