create database `blog`;
create table if not exists `papers` (
	`id` int(11) auto_increment primary key,
	`title` varchar(255) not null unique,
	`content` text not null,
	`author` varchar(255) not null,
	`last_change_time` timestamp default now(),
	`create_time` varchar(255) not null,
	`tags` varchar(255) not null,
	`hash` varchar(64) not null unique,
	key `index_hash`(`hash`)
);
