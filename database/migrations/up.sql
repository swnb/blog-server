create table  if not exists `paper` (
	`id` int(11) auto_increment primary key,
	`title` varchar(255) not null,
	`content` text not null,
	`author` varchar(255) not null,
	`last_change_time` timestamp default now(),
	`create_time` timestamp default now(),
	`tags` varchar(255) not null,
);
