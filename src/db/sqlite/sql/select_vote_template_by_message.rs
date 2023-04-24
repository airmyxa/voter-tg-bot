/*
?1 - chat_id
?2 - message_id
 */

pub static SELECT_VOTE_TEMPLATE_BY_MESSAGE: &'static str = "\
select vt.id, vt.invoke_command \
from vote_templates as vt \
join votes as v on v.template_id = vt.id \
where v.chat_id = ?1 and message_id = ?2;\
";
