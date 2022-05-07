create table tenants (
    tenant_id uuid not null,
    active boolean not null default false,
    created timestamp without time zone not null default(now() at time zone 'utc'),
    currency_alpha_3 char(3) not null,

    constraint pk_accounting_tenants
        primary key (tenant_id, currency_alpha_3),
    constraint fk_accounting_tenants_1
        foreign key (tenant_id)
        references tenants.tenants (id)
        on delete restrict
    -- constraint fk_accounting_tenants_2
    --     foreign key (currency_id)
    --     references common.currencies (currency_numeric)
    --     on delete restrict
);