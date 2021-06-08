-- Your SQL goes here
insert into users (username, password, role) values (
    'doran', 
    '$argon2id$v=19$m=65536,t=2,p=1$DiU2oghy2pOSq2mdvAF5Eg$V6K0U/txzPHJ/XcDdAySmMDhjtNPHkd7WeYqUCSoaxg', 
    'Student'
);

insert into users (username, password, role) values (
    'bastien', 
    '$argon2id$v=19$m=65536,t=2,p=1$5DsqDO/s+sopeSP3C9tpgg$Xm7vlHsL8XwfXJsX+sNP+2t+w54X2sW7N3g+HmT3TKQ', 
    'Student'
);

insert into users (username, password, role) values (
    'alexandre', 
    '$argon2id$v=19$m=65536,t=2,p=1$zB7kZezaLC71N6r/WAJCQA$Tz0Pnn7kEwmAtUARo9A7MI4jDhHGEQxVQ477kDMyGvk', 
    'Teacher'
);

insert into users (username, password, role) values (
    'rene', 
    '$argon2id$v=19$m=65536,t=2,p=1$1RyI7dOjQoLHgqfyH08z0A$xnO2W1joY6an/Ufus8eeXZEQBzA5y5mhRUIRrjnP5q0', 
    'Teacher'
);