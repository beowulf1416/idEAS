create table item_categories (
    id uuid not null,
    active boolean not null default false,
    created timestamp without time zone not null default(now() at time zone 'utc'),
    tenant_id uuid not null,
    name varchar(200) not null,
    description text,

    constraint pk_item_categories
        primary key (id),
    constraint fk_item_categories_1
        foreign key (tenant_id)
        references tenants.tenants (id)
        on delete restrict
);