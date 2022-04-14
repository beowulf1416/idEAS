create table roles (
    id uuid not null,
    tenant_id uuid not null,
    active boolean not null default false,
    created timestamp without time zone not null default(now() at time zone 'utc'),
    name varchar(100) not null,
    description text not null,

    constraint pk_roles
        primary key (id),
    constraint u_roles_1
        unique (tenant_id, name),
    constraint fk_roles_1
        foreign key (tenant_id)
        references tenants.tenants (id)
        on delete restrict
        on update restrict
);

comment on table roles is 'table of roles';