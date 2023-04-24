/*
?1 - invoke_command
 */

pub static SELECT_VOTE_TEMPLATE_BY_INVOKE_COMMAND: &'static str = "\
select id, invoke_command \
from vote_templates \
where invoke_command = ?1;\
";
