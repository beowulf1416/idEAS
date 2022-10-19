create table items (
    id uuid not null,
    client_id uuid not null,
    active boolean not null default false,
    created timestamp without time zone not null default(now() at time zone 'utc'),

    name text not null,
    description text,

    sku varchar(50),
    upc varchar(50),

    dimension_id smallint not null,
    uom_id smallint not null,

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
        foreign key (client_id)
        references client.clients (id)
        on delete restrict,
    constraint fk_items_2
        foreign key (dimension_id)
        references common.dimensions (id)
        on delete restrict,
    constraint fk_items_3
        foreign key (uom_id)
        references common.uom (id)
        on delete restrict,

    constraint chk_items_1
        check (volume > 0),
    constraint chk_items_2
        check (weight > 0),
    constraint chk_items_3
        check (shelf_width > 0),
    constraint chk_items_4
        check (shelf_height > 0),
    constraint chk_items_5
        check (shelf_depth > 0)
);