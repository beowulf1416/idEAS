create table accounts (
    id uuid not null,
    tenant_id uuid not null,
    active boolean not null default false,
    created timestamp without time zone not null default(now() at time zone 'utc'),
    type_id int not null,
    name varchar(200) not null,
    description text not null,

    constraint pk_accounts
        primary key (id),
    constraint u_accounts_1
        unique (tenant_id, name),
    constraint fk_accounts_1
        foreign key (tenant_id)
        references tenants.tenants (id)
        on delete restrict,
    constraint fk_accounts_2
        foreign key (type_id)
        references accounting.account_types (id)
        on delete restrict
);