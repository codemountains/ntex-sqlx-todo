create table if not exists todos (
    id serial,
    title text not null,
    status text not null default 'working',
    constraint pk_todos_id primary key (id)
);
