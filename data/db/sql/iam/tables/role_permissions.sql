create table role_permissions (
    active boolean not null default false,
    created timestamp without time zone not null default(now() at time zone 'utc'),
    role_id uuid not null,
    permission_id bigint not null,

    constraint pk_role_permissions
        primary key (role_id, permission_id),
    constraint fk_role_permissions_1
        foreign key (role_id)
        references iam.roles (id)
        on delete restrict,
    constraint fk_role_permissions_2
        foreign key (permission_id)
        references iam.permissions (id)
        on delete restrict
);

comment on table role_permissions is 'table mapping assigned permissions to roles';