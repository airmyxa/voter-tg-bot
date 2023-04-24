create table vote_templates (
    id bigserial primary key,
    invoke_command text not null,

    unique(invoke_command)
);