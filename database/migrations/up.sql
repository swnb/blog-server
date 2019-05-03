create table  if not exists `papers` (
	`id` int(11) auto_increment primary key,
	`title` varchar(255) not null unique,
	`content` text not null,
	`author` varchar(255) not null,
	`last_change_time` timestamp default now(),
	`create_time` timestamp default now(),
	`tags` varchar(255) not null,
	`index_hash` varchar(64) not null unique,
	key `index_some`(`index_hash`)
);
