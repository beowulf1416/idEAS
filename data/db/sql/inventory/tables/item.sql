create table items (
    id uuid not null,
    active boolean not null default false,
    created timestamp without time zone not null default(now() at time zone 'utc'),
    tenant_id uuid not null,
    category_id uuid not null,
    name varchar(200) not null,
    description text,

    constraint pk_items
        primary key (id),
    constraint fk_items_1
        foreign key (tenant_id)
        references tenants.tenants (id)
        on delete restrict,
    constraint fk_items_2
        foreign key (category_id)
        references inventory.items_category (id)
        on delete restrict
);