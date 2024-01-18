create table "user" (
    id serial primary key,
    username varchar(64) not null unique,
    password varchar(64) not null
);

create table login (
    id serial primary key,
    at timestamp with time zone not null,
    user_id int not null references "user" (id) on delete cascade,
    key varchar(36) unique
)