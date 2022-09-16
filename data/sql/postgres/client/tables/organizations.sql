create table organizations (
    id uuid not null,
    client_id uuid not null,
    active boolean not null,
    created timestamp without time zone not null default(now() at time zone 'utc'),

    name varchar(300) not null,
    description text,

    constraint pk_organizations
        primary key (id),

    constraint fk_organizations_1
        foreign key (client_id)
        references client.clients (id)
        on delete restrict
);


create table organizations_scd (
    org_id uuid not null,
    active boolean not null,
    created timestamp without time zone not null default(now() at time zone 'utc'),

    name varchar(300) not null,
    description text,

    constraint fk_organizations_scd
        foreign key (org_id)
        references client.organizations (id)
        on delete restrict
);