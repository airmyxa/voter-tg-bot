create table vote_templates
(
    id             integer primary key autoincrement,
    invoke_command text not null,

    unique (invoke_command)
);