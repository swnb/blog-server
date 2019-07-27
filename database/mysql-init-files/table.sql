create table papers (
    id uuid primary key default uuid_generate_v1(),
    title unique not null varchar(50),
    author not null varchar(50),
    content text,
    tags varchar(50) array,
    create_at timestamp not null default now(),
    change_records timestamp array,
    is_draft boolean default true,
    is_del boolean default false
);
