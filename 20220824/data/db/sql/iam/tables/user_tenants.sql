create table user_tenants (
    active boolean not null default false,
    created timestamp without time zone not null default(now() at time zone 'utc'),
    user_id uuid not null,
    tenant_id uuid not null,

    constraint pk_user_tenants
        primary key (user_id, tenant_id),
    constraint fk_user_tenants_1
        foreign key (user_id)
        references iam.users (id)
        on delete restrict,
    constraint fk_user_tenants_2
        foreign key (tenant_id)
        references tenants.tenants (id)
        on delete restrict
);

comment on table user_tenants is 'table mapping users to tenants';