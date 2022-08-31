create table item_locations (
    id uuid not null,
    domain_id uuid not null,
    active boolean not null default false,
    created timestamp without time zone not null default(now() at time zone 'utc'),

    item_id uuid not null,
    location_id uuid not null,

    batch_id varchar(30),
    lot_id varchar(30),
    
    expiry date,

    balance numeric(12, 3),
    uom_id bigint,

    constraint pk_item_locations
        primary key (id),

    constraint fk_item_locations_1
        foreign key (domain_id)
        references domain.domains (id)
        on delete restrict
);


create table item_locations_scd (
    item_location_id uuid not null,
    created timestamp without time zone not null default(now() at time zone 'utc'),

    active boolean not null default false,

    item_id uuid not null,
    location_id uuid not null,

    batch_id varchar(30),
    lot_id varchar(30),
    
    expiry date,

    balance numeric(12, 3),
    uom_id bigint,


    constraint fk_item_locations_scd_1
        foreign key (item_location_id)
        references item_locations (id)
        on delete restrict
);