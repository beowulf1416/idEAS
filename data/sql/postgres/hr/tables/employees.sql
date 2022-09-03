create table employees (
    id uuid not null,
    client_id uuid not null,
    active boolean not null default false,
    created timestamp without time zone not null default(now() at time zone 'utc'),

    people_id uuid not null,

    constraint pk_employees
        primary key (id),

    constraint fk_employees_1
        foreign key (client_id)
        references client.clients (id)
        on delete restrict,
    constraint fk_employees_2
        foreign key (people_id)
        references crm.people (id)
        on delete restrict
);