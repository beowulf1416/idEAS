create table item_substitutes (
    client_id uuid not null,
    active boolean not null default false,
    created timestamp without time zone not null default(now() at time zone 'utc'),

    item_id uuid not null,
    substitute_item_id uuid not null,

    constraint pk_item_substitutes
        primary key (client_id, item_id, substitute_item_id),

    constraint fk_item_substitutes_1
        foreign key (client_id)
        references client.clients (id)
        on delete restrict,
    constraint fk_item_substitutes_2
        foreign key (item_id)
        references inventory.items (id)
        on delete restrict
);

comment on table item_substitutes is 'substitutable items';