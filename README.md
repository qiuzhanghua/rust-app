# Rust App Boilerplate

1. support log4rs
2. support config file
3. support handlebars as template
4. support front and back-end separation(insert html/js in binary)


## dir/file structure
- config
     
      config files
      
- static

      template files
      
- public

      React/Vue/Angular files
     
- src

      rust files

- log

      log files
            
- Cargo.toml


## Diesel

diesel setup --database-url=mysql://app:app@localhost:3306/app
diesel setup --database-url=postgresql://app:app@localhost:5432/app

diesel migration generate create_users

diesel migration run --database-url=mysql://app:app@localhost:3306/app
diesel migration run --database-url=postgresql://app:app@localhost:5432/app

diesel migration redo --database-url=mysql://app:app@localhost:3306/app
diesel migration redo --database-url=postgresql://app:app@localhost:5432/app

pg:
```sql
create table users
(
	id SERIAL PRIMARY KEY,
	name VARCHAR(80) not null,
	email VARCHAR(128) not null,
	enabled boolean default false not null
)

drop table users
```

mysql
```sql
create table users
(
	id int auto_increment,
	name varchar(80) not null,
	email varchar(128) not null,
	enabled boolean default false not null,
	constraint users_pk
		primary key (id)
);
drop table if exists users;
```

```sql
create table users
(
	id bigserial
		constraint users_pk
			primary key,
	name varchar(80) not null,
	email varchar(128) not null,
	enabled boolean default false not null
);
```
