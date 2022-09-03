create table time_entries (
    id uuid not null,
    client_id uuid not null,
    active boolean not null default false,
    created timestamp without time zone not null default(now() at time zone 'utc'),

    employee_id uuid not null,

    constraint pk_time_entries
        primary key (id),

    constraint fk_time_entries_1
        foreign key (client_id)
        references client.clients (id)
        on delete restrict,
    constraint fk_time_entries_2
        foreign key (employee_id)
        references hr.employees (id)
        on delete restrict
);