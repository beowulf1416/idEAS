create table people (
    id uuid not null,
    domain_id uuid not null,
    active boolean not null default false,
    created timestamp without time zone not null default(now() at time zone 'utc'),

    given_name varchar(200) not null,
    middle_name varchar(200),
    family_name varchar(200) not null,

    prefix varchar(100),
    suffix varchar(100),

    constraint pk_people
        primary key (id)
);