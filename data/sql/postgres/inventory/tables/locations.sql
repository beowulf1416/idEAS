create table locations (
    id uuid not null,
    client_id uuid not null,
    active boolean not null default false,
    created timestamp without time zone not null default(now() at time zone 'utc'),

    aisle varchar(30),
    shelf varchar(30),
    bin varchar(30),
    level varchar(30),
    floor varchar(30),

    constraint pk_locations
        primary key (id),

    constraint fk_locations_1
        foreign key (client_id)
        references client.clients (id)
        on delete restrict
);