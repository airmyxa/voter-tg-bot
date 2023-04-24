/*
?1 - chat_id
?2 - message_id
?3 - text
?4 - template_id
 */

pub static INSERT_VOTE: &'static str = "\
insert into votes (chat_id, message_id, text, template_id) \
values (?1, ?2, ?3, ?4) \
on conflict do nothing;\
";
