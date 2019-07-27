create table papers (
    id uuid primary key default uuid_generate_v1(),
    title varchar(50) not null unique,
    author varchar(50) not null,
    content text not null,
    tags varchar(50) array,
    create_at timestamp not null default now(),
    change_records timestamp array,
    is_draft boolean default true,
    is_del boolean default false
);
