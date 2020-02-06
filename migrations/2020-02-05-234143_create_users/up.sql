-- Your SQL goes here

create table users
(
	id int auto_increment,
	name varchar(80) not null,
	email varchar(128) not null,
	enabled boolean default false not null,
	constraint users_pk
		primary key (id)
);

