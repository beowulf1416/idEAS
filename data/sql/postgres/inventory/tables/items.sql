create table items (
    id uuid not null,
    domain_id uuid not null,
    active boolean not null default false,
    created timestamp without time zone not null default(now() at time zone 'utc'),

    name text not null,
    description text,

    sku varchar(50),
    upc varchar(50),

    dimension_id tinyint not null,
    uom_id tinyint not null,

    volume numeric(12, 3),
    weight numeric(12, 3),
    shelf_width numeric(10, 3),
    shelf_height numeric(10, 3),
    shelf_depth numeric(10, 3),

    perishable boolean not null default false,
    stocked boolean not null default false,
    purchased boolean not null default false,
    sold boolean not null default false,
    manufactured boolean not null default false,

    constraint pk_items
        primary key (id),
    
    constraint fk_items_1
        foreign key (domain_id)
        references domain.domains (id)
        on delete restrict
);