create table identifiers (
    id uuid not null,
    domain_id uuid not null,
    active boolean not null default false,
    created timestamp without time zone not null default(now() at time zone 'utc'),

    type_id uuid not null,
    people_id uuid not null,
    value varchar(300) not null

    constraint pk_identifiers
        primary key (id),

    constraint u_identifier_types_1
        unique (domain_id, type_id, people_id),

    constraint fk_identifier_types_1
        foreign key (domain_id)
        references domain.domains (id)
        on delete restrict,
    constraint fk_identifier_types_2
        foreign key (type_id)
        references identifier_types (id)
        on delete restrict,
    constraint fk_identifier_types_3
        foreign key (people_id)
        references people (id)
        on delete restrict
);