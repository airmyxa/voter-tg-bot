/*
?1 - invoke_command
 */

pub static INSERT_VOTE_TEMPLATE: &'static str = "\
insert into vote_templates (invoke_command) \
values (?1) on conflict do nothing \
returning id;\
";
