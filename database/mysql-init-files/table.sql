use blog;
create table if not exists `papers` (
	`id` int(11) auto_increment primary key,
	`title` varchar(255) not null unique,
	`content` text not null,
	`author` varchar(255) not null,
	`last_change_time` timestamp default now(),
	`create_time` varchar(255) not null,
	`hash` varchar(64) not null unique,
	key `index_hash_content`(`hash`,`content`(255))
);

create table if not exists `paper_tags` (
	`id` varchar(64) not null,
	`tag` varchar(255) not null,
	primary key(`id`,`tag`),
	unique key `index_tag_id` (`tag`,`id`)
);
