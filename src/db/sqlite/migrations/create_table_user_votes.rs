pub static CREATE_TABLE_USER_VOTES: &'static str = "
create table user_votes (
   id integer primary key autoincrement,
   vote_id bigint references votes (id),
   user_name text not null,
   query_data text not null,

   unique(vote_id, user_name)
);
";
