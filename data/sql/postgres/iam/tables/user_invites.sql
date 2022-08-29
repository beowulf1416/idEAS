create table user_invite (
    id uuid not null,
    created timestamp without time zone not null default(now() at time zone 'utc'),

    client_id uuid not null,
    token varchar(20) not null,
    email common.email_address,

    constraint pk_user_invite
        primary key (id),

    constraint fk_user_invite_1
        foreign key (client_id)
        references client.clients (id)
        on delete restrict
);