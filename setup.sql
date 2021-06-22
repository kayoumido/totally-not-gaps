create table users (
    id serial primary key,
    username varchar not null,
    password varchar not null,
    role varchar not null
);

create table grades (
    id serial primary key,
    grade real not null,
    student_id integer not null,

    constraint fk_student
      foreign key(student_id) 
      references users(id)
);

insert into users (username, password, role) values (
    'doran', 
    '$argon2id$v=19$m=65536,t=2,p=1$dR1nEDVxBWoSGxqiIkZSaw$/9jBDLO0UWAtjAI7d/XEGg/Maj+EtngoS1LWJQuCDis', 
    'Student'
);

insert into users (username, password, role) values (
    'bastien', 
    '$argon2id$v=19$m=65536,t=2,p=1$YpyDon7L5IJRpLfbswwvbg$H9KZOwWl0fEs8f7//roju7Am+pko8CU/12h5FAnmAWk', 
    'Student'
);

insert into users (username, password, role) values (
    'alexandre', 
    '$argon2id$v=19$m=65536,t=2,p=1$fYVO/iOOriQroW7PfADQPQ$9uJNYqxJ7Fgd27GHrAJNCu/PbwsBv3DOG1it78LSVmA', 
    'Teacher'
);

insert into users (username, password, role) values (
    'rene', 
    '$argon2id$v=19$m=65536,t=2,p=1$iiIdnjrtNMjaE3A/aR17IQ$8mhLxqoGjA9dbkgJD3QYYFCkASEdI00ZkH/OrsQYkHY', 
    'Teacher'
);