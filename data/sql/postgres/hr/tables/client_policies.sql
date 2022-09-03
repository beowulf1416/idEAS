create table domain_policies (
    id uuid not null,
    client_id uuid not null,
    active boolean not null default false,
    created timestamp without time zone not null default(now() at time zone 'utc'),

    minimum_hourly_wage money not null default 0,

    constraint pk_client_policies
        primary key (id),

    constraint fk_employees_1
        foreign key (client_id)
        references client.clients (id)
        on delete restrict
);