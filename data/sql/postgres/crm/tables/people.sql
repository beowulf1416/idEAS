create table people (
    id uuid not null,
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


create table people_scd (
    people_id uuid not null,
    active boolean not null default false,
    created timestamp without time zone not null default(now() at time zone 'utc'),

    given_name varchar(200) not null,
    middle_name varchar(200),
    family_name varchar(200) not null,

    prefix varchar(100),
    suffix varchar(100),

    constraint fk_people_scd_1
        foreign key (people_id)
        references people (id)
        on delete restrict
);