create table identifier_types (
    id uuid not null,
    domain_id uuid not null,
    active boolean not null default false,
    created timestamp without time zone not null default(now() at time zone 'utc'),

    name varchar(30) not null,
    description text not null,

    constraint pk_identifier_types
        primary key (id),

    constraint u_identifier_types_1
        unique (domain_id, name),

    constraint fk_identifier_types_1
        foreign key (domain_id)
        references domain.domains (id)
        on delete restrict
);