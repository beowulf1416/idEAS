create table user_registration (
    id uuid not null,
    created timestamp without time zone not null default(now() at time zone 'utc'),

    email common.email_address,

    expires timestamp without time zone not null default(now() at time zone 'utc' + interval '1' day),
    completed timestamp without time zone,

    constraint pk_user_registration
        primary key (id)
);