create table items (
    id uuid not null,
    domain_id uuid not null,
    active boolean not null default false,
    created timestamp without time zone not null default(now() at time zone 'utc'),

    name text not null,
    description text,

    sku varchar(200),
    

    perishable boolean not null default false,

    constraint pk_items
        primary key (id),
    
    constraint fk_items_1
        foreign key (domain_id)
        references domain.domains (id)
        on delete restrict
);