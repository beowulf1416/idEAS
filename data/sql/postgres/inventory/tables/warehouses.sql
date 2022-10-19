create table warehouses (
    id uuid not null,
    client_id uuid not null,
    active boolean not null default false,
    created timestamp without time zone not null default(now() at time zone 'utc'),

    name varchar(200) not null,
    description text,

    constraint pk_warehouses
        primary key (id),

    constraint u_warehouses_1
        unique (client_id, name),

    constraint fk_warehouses_1
        foreign key (client_id)
        references client.clients (id)
        on delete restrict
);