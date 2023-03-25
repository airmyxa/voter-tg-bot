/*
?1 - chat_id
?2 - message_id
 */

pub static SELECT_VOTE: &'static str = "\
select id, chat_id, message_id, text \
from votes \
where chat_id = ?1 and message_id = ?2;\
";
