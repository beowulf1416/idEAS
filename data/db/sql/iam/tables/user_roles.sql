create table user_roles (
    active boolean not null default false,
    created timestamp without time zone not null default(now() at time zone 'utc'),
    user_id uuid not null,
    role_id uuid not null,

    constraint pk_user_roles
        primary key (user_id, role_id),
    constraint fk_user_roles_1
        foreign key (user_id)
        references iam.users (id)
        on delete restrict,
    constraint fk_user_roles_2
        foreign key (role_id)
        references iam.roles (id)
        on delete restrict
);

comment on table user_roles is 'table mapping assigned roles to users';