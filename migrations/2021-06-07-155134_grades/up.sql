-- Your SQL goes here
create table grades (
    id serial primary key,
    grade real not null,
    student_id integer not null,

    constraint fk_student
      foreign key(student_id) 
      references users(id)
);