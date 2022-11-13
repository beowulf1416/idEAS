create table item_components (
    client_id uuid not null,
    active boolean not null default false,
    created timestamp without time zone not null default(now() at time zone 'utc'),

    item_id uuid not null,
    qty decimal(12,3) not null,
    dimension_id smallint not null,
    uom_id bigint not null,

    constraint pk_item_components
        primary key (client_id, item_id),

    constraint fk_item_components_1
        foreign key (client_id)
        references client.clients (id)
        on delete restrict,
    constraint fk_item_components_2
        foreign key (item_id)
        references inventory.items (id)
        on delete restrict,
    constraint fk_item_components_3
        foreign key (dimension_id)
        references common.dimensions (id)
        on delete restrict,
    constraint fk_item_components_4
        foreign key (uom_id)
        references common.uom (id)
        on delete restrict
);

comment on table item_components is 'table of inventory item components';