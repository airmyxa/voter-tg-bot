/*
?1 - chat_id
?2 - message_id
?3 - user_name
?4 - choice
 */

pub static UPSERT_USER_VOTES: &'static str = "\
insert into user_votes (vote_id, user_name, choice) \
select v.id, ?3, ?4 \
from votes as v \
where v.chat_id = ?1 and v.message_id = ?2 \
on conflict (vote_id, user_name) do update \
set choice = ?4; \
";
