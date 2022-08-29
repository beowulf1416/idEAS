create table user_registration (
    id uuid not null,
    created timestamp without time zone not null default(now() at time zone 'utc'),

    token varchar(20) not null,
    email common.email_address,

    expires datetime not null,
    completed datetime not null,

    constraint pk_user_registration
        primary key (id)
);