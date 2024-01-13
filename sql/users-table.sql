create table if not exists users (
	id serial primary key,
	username varchar(60) unique not null,
	password varchar(69) not null,
	email varchar(255) unique not null
);
