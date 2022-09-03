create table tasks (
    id uuid not null,
    client_id uuid not null,
    active boolean not null default false,
    created timestamp without time zone not null default(now() at time zone 'utc'),

    name varchar(200) not null,
    description text,

    pay_rate_type_id tinyint not null,

    
);