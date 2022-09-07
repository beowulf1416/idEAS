create table identifiers (
    id uuid not null,
    client_id uuid not null,
    active boolean not null default false,
    created timestamp without time zone not null default(now() at time zone 'utc'),

    type_id uuid not null,
    people_id uuid not null,
    value varchar(300) not null,

    constraint pk_identifiers
        primary key (id),

    constraint u_identifiers_1
        unique (client_id, type_id, people_id),

    constraint fk_identifiers_1
        foreign key (client_id)
        references client.clients (id)
        on delete restrict,
    constraint fk_identifiers_2
        foreign key (type_id)
        references identifiers (id)
        on delete restrict,
    constraint fk_identifiers_3
        foreign key (people_id)
        references people (id)
        on delete restrict
);