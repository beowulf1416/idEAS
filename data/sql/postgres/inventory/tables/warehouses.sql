create table warehouses (
    id uuid not null,
    domain_id uuid not null,
    active boolean not null default false,
    created timestamp without time zone not null default(now() at time zone 'utc'),

    name varchar(200) not null,
    description text,

    constraint pk_warehouses
        primary key (id),

    constraint fk_warehouses_1
        foreign key (domain_id)
        references domain.domains (id)
        on delete restrict
);