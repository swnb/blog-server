create table papers (
    id uuid primary key default uuid_generate_v1(),
    title varchar(50) not null unique,
    author varchar(50) not null,
    content text not null,
    tags text array not null,
    create_at timestamptz not null default now(),
    change_records timestamptz array not null default array[]::timestamptz[],
    is_draft boolean default true,
    is_del boolean default false
);