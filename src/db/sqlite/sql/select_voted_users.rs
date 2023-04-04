/*
?1 - chat_id
?2 - message_id
 */

pub static SELECT_VOTED_USERS: &'static str = "\
select uv.vote_id, uv.user_name, uv.query_data \
from user_votes as uv \
join votes as v on uv.vote_id = v.id \
where v.chat_id = ?1 and v.message_id = ?2;\
";
