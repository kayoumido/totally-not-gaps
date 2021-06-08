-- Your SQL goes here
create table users (
    id serial primary key,
    username varchar not null,
    password varchar not null,
    role varchar not null
);