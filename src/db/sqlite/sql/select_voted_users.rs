/*
?1 - chat_id
?2 - message_id
 */

pub static SELECT_VOTED_USERS: &'static str = "\
select user_name \
from votes \
where chat_id = ?1 and message_id = ?2;\
";
