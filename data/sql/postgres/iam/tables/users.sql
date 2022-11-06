create table users (
    id uuid not null,
    active boolean not null default false,
    created timestamp without time zone not null default(now() at time zone 'utc'),

    email common.email_address not null,
    pw text not null,

    constraint pk_users
        primary key (id),

    constraint u_users_1
        unique (email)
);

comment on table users is 'table of user accounts';