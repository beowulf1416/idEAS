create table user_registration (
    id uuid not null,
    created datetime not null,

    expires datetime not null,
    completed datetime not null,

    constraint pk_user_registration
        primary key (id)
);