CREATE TABLE user (
    id int not null auto_increment,
    name varchar(50) not null,
    email varchar(50) not null,
    subject varchar(50) not null,
    message varchar(50) not null,
    primary key (id)
);
