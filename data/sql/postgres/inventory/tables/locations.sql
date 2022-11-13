create table locations (
    id uuid not null,
    client_id uuid not null,
    active boolean not null default false,
    created timestamp without time zone not null default(now() at time zone 'utc'),

    warehouse_id uuid not null,
    aisle varchar(30),
    shelf varchar(30),
    bin varchar(30),
    pallet varchar(30),
    level varchar(30),
    floor varchar(30),

    constraint pk_locations
        primary key (id),

    constraint u_locations_1
        unique (warehouse_id, aisle, shelf, bin, pallet, level, floor),

    constraint fk_locations_1
        foreign key (client_id)
        references client.clients (id)
        on delete restrict,
    constraint fk_locations_2
        foreign key (warehouse_id)
        references inventory.warehouses (id)
        on delete restrict,

    constraint chk_locations_1
        check (
            concat(
                aisle,
                shelf,
                bin,
                pallet,
                level,
                floor
            ) <> ''
        )
);